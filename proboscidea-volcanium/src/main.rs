use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead, BufReader},
};

#[derive(Debug)]
struct GraphNode<T> {
    value: T,
    neighbors: HashSet<String>,
}

#[derive(Debug)]
struct Graph<T> {
    nodes: HashMap<String, GraphNode<T>>,
}

fn parse_cave<R: BufRead>(r: BufReader<R>) -> Graph<u64> {
    let re =
        Regex::new(r"Valve (.+) has flow rate=(\d+); tunnels? leads? to valves? ((.+)(, .+)*)")
            .unwrap();
    let mut nodes = HashMap::new();
    for line in r.lines() {
        let l = line.unwrap();
        let cap = re.captures(l.as_str()).unwrap();
        let key = cap.get(1).unwrap().as_str();
        let rate = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();
        let neighbors = cap
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|v| String::from(v))
            .collect();
        let node = GraphNode {
            value: rate,
            neighbors: neighbors,
        };
        nodes.insert(String::from(key), node);
    }
    Graph { nodes: nodes }
}

fn find_shortest_paths(graph: &Graph<u64>) -> HashMap<String, HashMap<String, u64>> {
    let mut paths = HashMap::new();
    for (src_key, src_node) in &graph.nodes {
        let mut shortest_paths = HashMap::new();
        let mut q = Vec::new();
        for next_key in &src_node.neighbors {
            q.push((next_key, graph.nodes.get(next_key).unwrap()));
        }
        let mut visited = HashSet::new();
        let mut counter: u64 = 1;
        loop {
            if q.len() == 0 {
                break;
            }
            let mut buffer = Vec::new();
            while q.len() > 0 {
                let (curr_key, curr_node) = q.remove(0);
                shortest_paths.insert(curr_key.clone(), counter);
                visited.insert(curr_key);
                for next_key in &curr_node.neighbors {
                    if !visited.contains(next_key) {
                        buffer.push((next_key, graph.nodes.get(next_key).unwrap()));
                    }
                }
            }
            q = buffer.clone();
            counter += 1;
        }
        paths.insert(src_key.clone(), shortest_paths);
    }
    paths
}

fn find_max_pressure_solo(
    key: &String,
    minutes_left: u64,
    graph: &Graph<u64>,
    shortest_paths: &HashMap<String, HashMap<String, u64>>,
    open_valve_nodes: HashSet<String>,
) -> u64 {
    if minutes_left == 0 {
        return 0;
    }
    if open_valve_nodes.len() == graph.nodes.len() {
        return 0;
    }
    let paths = shortest_paths.get(key).unwrap();
    let mut vs = Vec::new();
    for next_key in graph.nodes.keys() {
        let next_node = graph.nodes.get(next_key).unwrap();
        let cost = *paths.get(next_key).unwrap();
        if minutes_left >= cost + 1 && !open_valve_nodes.contains(next_key) && next_node.value > 0 {
            let next_minutes_left = minutes_left - cost - 1;
            let mut open_valve_nodes = open_valve_nodes.clone();
            open_valve_nodes.insert(next_key.clone());
            let v = next_minutes_left * next_node.value
                + find_max_pressure_solo(
                    next_key,
                    next_minutes_left,
                    graph,
                    shortest_paths,
                    open_valve_nodes,
                );
            vs.push(v);
        }
    }
    match vs.iter().max() {
        Some(v) => *v,
        None => 0,
    }
}

// This is slow because I am bad
fn find_max_pressure_duo(
    key_1: &String,
    key_2: &String,
    minutes_left_1: u64,
    minutes_left_2: u64,
    graph: &Graph<u64>,
    shortest_paths: &HashMap<String, HashMap<String, u64>>,
    open_valve_nodes: HashSet<String>,
) -> u64 {
    if minutes_left_1 == 0 && minutes_left_2 == 0 {
        return 0;
    }
    if open_valve_nodes.len() == graph.nodes.len() {
        return 0;
    }
    let paths = shortest_paths.get(key_1).unwrap();
    let mut vs = Vec::new();

    // Choose to stop and release the elephant
    vs.push(find_max_pressure_solo(
        key_2,
        minutes_left_2,
        graph,
        shortest_paths,
        open_valve_nodes.clone(),
    ));

    // Choose to keep going
    for next_key in graph.nodes.keys() {
        let next_node = graph.nodes.get(next_key).unwrap();
        let cost = *paths.get(next_key).unwrap();
        if minutes_left_1 >= cost + 1 && !open_valve_nodes.contains(next_key) && next_node.value > 0
        {
            let next_minutes_left = minutes_left_1 - cost - 1;
            let mut open_valve_nodes = open_valve_nodes.clone();
            open_valve_nodes.insert(next_key.clone());
            let v = next_minutes_left * next_node.value
                + find_max_pressure_duo(
                    next_key,
                    key_2,
                    next_minutes_left,
                    minutes_left_2,
                    graph,
                    shortest_paths,
                    open_valve_nodes.clone(),
                );
            vs.push(v);
        }
    }
    match vs.iter().max() {
        Some(v) => *v,
        None => 0,
    }
}

fn main() {
    let stdin = io::stdin();
    let r = BufReader::new(stdin.lock());
    let cave = parse_cave(r);
    let shortest_paths = find_shortest_paths(&cave);
    println!(
        "Max pressure solo: {}",
        find_max_pressure_solo(
            &String::from("AA"),
            30,
            &cave,
            &shortest_paths,
            HashSet::new(),
        )
    );
    println!(
        "Max pressure with elephant: {}",
        find_max_pressure_duo(
            &String::from("AA"),
            &String::from("AA"),
            26,
            26,
            &cave,
            &shortest_paths,
            HashSet::new(),
        )
    );
}
