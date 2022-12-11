use super::lines::LineParse;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Item {
  Dir { name: String },
  File { name: String, size: u64 },
}

#[derive(Debug, PartialEq)]
pub struct Node<T> {
  pub value: Item,
  children: Vec<Rc<RefCell<Node<T>>>>,
}

impl Node<Item> {
  pub fn total_size(&self) -> u64 {
    match &self.value {
      Item::Dir { name } => {
        let mut s = 0;
        for child_node in &self.children {
          let child_node = child_node.borrow();
          s += child_node.total_size();
        }
        s
      }
      Item::File { name, size } => *size,
    }
  }
}

pub fn parse(line_parses: Vec<LineParse>) -> Option<Rc<RefCell<Node<Item>>>> {
  let mut root_rc_opt: Option<Rc<RefCell<Node<Item>>>> = None;
  let mut line_idx = 0;
  let mut node_history: Vec<Rc<RefCell<Node<Item>>>> = Vec::new();
  let mut is_ls = false;
  while line_idx < line_parses.len() {
    let line_parse = &line_parses[line_idx];
    match line_parse {
      LineParse::CD { path } => {
        is_ls = false;
        match path.as_str() {
          "/" => {
            let n = Node {
              value: Item::Dir {
                name: String::from("/"),
              },
              children: Vec::new(),
            };
            root_rc_opt = Some(Rc::new(RefCell::new(n)));
            node_history.clear();
            node_history.push(root_rc_opt.clone().unwrap());
          }
          ".." => {
            node_history.pop();
          }
          _ => {
            let curr_node_rc = node_history.last().unwrap().clone();
            let curr_node = curr_node_rc.borrow();
            match &curr_node.value {
              Item::Dir { name } => {
                let child_node_opt = curr_node.children.iter().find(|&v| {
                  let node = v.borrow();
                  let child_name = match &node.value {
                    Item::Dir { name } => name,
                    Item::File { name, size } => name,
                  };
                  child_name == path
                });
                match child_node_opt {
                  Some(child_node) => {
                    let child_node = child_node.clone();
                    node_history.push(child_node);
                  }
                  None => {
                    panic!("cannot cd into unknown directory")
                  }
                }
              }
              _ => {
                panic!("cannot cd into non-directory")
              }
            }
          }
        }
      }
      LineParse::LS => {
        is_ls = true;
      }
      LineParse::Dir { name } => {
        let dir_name = name;
        if is_ls {
          let curr_node_rc = node_history.last().unwrap().clone();
          let mut curr_node = curr_node_rc.borrow_mut();
          match &curr_node.value {
            Item::Dir { name } => {
              let child_node = Rc::new(RefCell::new(Node {
                value: Item::Dir {
                  name: String::from(dir_name),
                },
                children: Vec::new(),
              }));
              curr_node.children.push(child_node.clone());
            }
            _ => {
              panic!("cannot add child to non-directory");
            }
          }
        }
      }
      LineParse::File { name, size } => {
        let file_name = name;
        if is_ls {
          let curr_node_rc = node_history.last().unwrap().clone();
          let mut curr_node = curr_node_rc.borrow_mut();
          match &curr_node.value {
            Item::Dir { name } => {
              let child_node = Rc::new(RefCell::new(Node {
                value: Item::File {
                  name: String::from(file_name),
                  size: *size,
                },
                children: Vec::new(),
              }));
              curr_node.children.push(child_node.clone());
            }
            _ => {
              panic!("cannot add child to non-directory");
            }
          }
        }
      }
    }
    line_idx += 1;
  }
  root_rc_opt
}

pub fn flatten<T>(root_rc: Rc<RefCell<Node<T>>>) -> Vec<Rc<RefCell<Node<T>>>>
where
  T: Debug,
{
  let mut nodes: Vec<Rc<RefCell<Node<T>>>> = Vec::new();
  let mut buffer: Vec<Rc<RefCell<Node<T>>>> = vec![root_rc];
  while buffer.len() > 0 {
    let mut frontier: Vec<Rc<RefCell<Node<T>>>> = Vec::new();
    for node in &buffer {
      nodes.push(node.clone());
      for child_node in &node.borrow().children {
        frontier.push(child_node.clone());
      }
    }
    buffer = frontier;
  }
  nodes
}

pub fn find_directories_lte_threshold(
  root_rc: Rc<RefCell<Node<Item>>>,
  threshold: u64,
) -> Vec<Rc<RefCell<Node<Item>>>> {
  let nodes = flatten(root_rc);
  nodes
    .iter()
    .filter(|&node_rc| {
      let node = node_rc.borrow();
      match &node.value {
        Item::Dir { name } => {
          let total_size = node.total_size();
          total_size <= threshold
        }
        Item::File { name, size } => false,
      }
    })
    .map(|node_rc| node_rc.clone())
    .collect::<Vec<Rc<RefCell<Node<Item>>>>>()
}

pub fn find_smallest_removable_directory(
  root: Rc<RefCell<Node<Item>>>,
  total_space: u64,
  target_space: u64,
) -> Option<Rc<RefCell<Node<Item>>>> {
  let mut node_rcs: Vec<Rc<RefCell<Node<Item>>>> = flatten(root.clone())
    .iter()
    .filter(|&node_rc| {
      let node = node_rc.borrow();
      match &node.value {
        Item::Dir { name } => true,
        Item::File { name, size } => false,
      }
    })
    .map(|node| node.clone())
    .collect();
  let used_space = root.borrow().total_size();
  node_rcs.sort_by(|a, b| {
    let a_size = a.borrow().total_size();
    let b_size = b.borrow().total_size();
    a_size.cmp(&b_size)
  });
  let available_space = total_space - used_space;
  for node_rc in node_rcs {
    if node_rc.borrow().total_size() + available_space >= target_space {
      return Some(node_rc);
    }
  }
  None
}
