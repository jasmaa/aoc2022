use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Clone, Debug)]
struct Rucksack {
    items: String,
}

impl Rucksack {
    fn new(items: String) -> Rucksack {
        return Rucksack { items: items };
    }

    fn find_common_compartment_item(&self) -> Option<char> {
        let compartment_size = self.items.len() / 2;
        let mut compartment1 = HashSet::new();
        for item in self.items[..compartment_size].chars() {
            compartment1.insert(item);
        }
        let mut compartment2 = HashSet::new();
        for item in self.items[compartment_size..].chars() {
            compartment2.insert(item);
        }
        let mut intersection = compartment1.intersection(&compartment2);
        match intersection.next() {
            Some(v) => Some(*v),
            None => None,
        }
    }
}

fn convert_item_to_priority(item: char) -> Option<i64> {
    if item >= 'a' && item <= 'z' {
        Some((item as i64) - ('a' as i64) + 1)
    } else if item >= 'A' && item <= 'Z' {
        Some((item as i64) - ('A' as i64) + 27)
    } else {
        None
    }
}

fn find_badge(rucksacks: &Vec<Rucksack>) -> Option<char> {
    match &rucksacks[..] {
        [r1, r2, r3] => {
            let mut h1 = HashSet::new();
            for item in r1.items.chars() {
                h1.insert(item);
            }
            let mut h2 = HashSet::new();
            for item in r2.items.chars() {
                h2.insert(item);
            }
            let mut h3 = HashSet::new();
            for item in r3.items.chars() {
                h3.insert(item);
            }
            let mut h12 = HashSet::new();
            for item in h1.intersection(&h2).enumerate() {
                h12.insert(*item.1);
            }
            let mut h123 = h12.intersection(&h3);
            match h123.next() {
                Some(v) => Some(*v),
                None => None,
            }
        }
        _ => None,
    }
}

fn find_total_rucksack_priority(rucksacks: &Vec<Rucksack>) -> i64 {
    rucksacks
        .iter()
        .map(|r| convert_item_to_priority(r.find_common_compartment_item().unwrap()).unwrap())
        .sum()
}

fn find_total_group_priority(groups: &Vec<Vec<Rucksack>>) -> i64 {
    groups
        .iter()
        .map(|rucksacks| {
            let badge = find_badge(rucksacks).unwrap();
            convert_item_to_priority(badge).unwrap()
        })
        .sum()
}

fn main() {
    let stdin = io::stdin();
    let mut rucksacks = Vec::new();
    let mut groups = vec![Vec::new()];
    let mut counter = 0;
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let items = l.clone();
        let rucksack = Rucksack::new(items);
        rucksacks.push(rucksack.clone());

        if counter == 3 {
            groups.push(Vec::new());
            counter = 0;
        }
        let idx = groups.len() - 1;
        groups[idx].push(rucksack.clone());
        counter += 1;
    }

    println!("Total rucksack priority: {}", find_total_rucksack_priority(&rucksacks));
    println!("Total group priority: {}", find_total_group_priority(&groups));
}
