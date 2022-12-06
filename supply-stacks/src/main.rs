use regex::Regex;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Crates {
    stacks: Vec<Vec<String>>,
}

impl Crates {
    fn new(lines: Vec<String>) -> Crates {
        let nums_re = Regex::new(r"\s\d+\s").unwrap();
        let n_crates = nums_re.find_iter(lines[lines.len() - 1].as_str()).count();
        let mut stacks: Vec<Vec<String>> = Vec::new();
        for _ in 0..n_crates {
            stacks.push(Vec::new());
        }
        for i in (0..(lines.len() - 1)).rev() {
            let line_chars: Vec<char> = lines[i].chars().collect();
            let mut p = 0;
            let mut counter = 0;
            while p < line_chars.len() {
                if line_chars[p] == '[' {
                    stacks[counter].push(String::from(line_chars[p + 1]));
                }
                p += 4;
                counter += 1;
            }
        }
        Crates { stacks: stacks }
    }

    fn top(&self) -> Vec<String> {
        let mut crates = Vec::new();
        for i in 0..self.stacks.len() {
            crates.push(self.stacks[i][self.stacks[i].len() - 1].clone());
        }
        crates
    }
}

struct CrateMover9000 {}

impl CrateMover9000 {
    fn new() -> CrateMover9000 {
        CrateMover9000 {}
    }

    fn execute(&self, crates: &mut Crates, ins: &Instruction) {
        for _ in 0..ins.quantity {
            let v = crates.stacks[ins.from].pop().unwrap();
            crates.stacks[ins.to].push(v);
        }
    }
}

struct CrateMover9001 {}

impl CrateMover9001 {
    fn new() -> CrateMover9001 {
        CrateMover9001 {}
    }

    fn execute(&self, crates: &mut Crates, ins: &Instruction) {
        let mut buffer = Vec::new();
        for _ in 0..ins.quantity {
            let v = crates.stacks[ins.from].pop().unwrap();
            buffer.push(v);
        }
        while buffer.len() > 0 {
            let v = buffer.pop().unwrap();
            crates.stacks[ins.to].push(v);
        }
    }
}

#[derive(Debug)]
struct Instruction {
    from: usize,
    to: usize,
    quantity: u64,
}

impl Instruction {
    fn new(line: String) -> Instruction {
        let re = Regex::new(r"^move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)$").unwrap();
        let cap = re.captures(line.as_str()).unwrap();
        Instruction {
            from: cap.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
            to: cap.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
            quantity: cap.get(1).unwrap().as_str().parse::<u64>().unwrap(),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut is_parsing_crates = true;
    let mut crate_lines: Vec<String> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.len() == 0 {
            is_parsing_crates = false;
        } else if is_parsing_crates {
            crate_lines.push(l)
        } else {
            let ins = Instruction::new(l);
            instructions.push(ins);
        }
    }
    let crates = Crates::new(crate_lines);

    let mut crate_mover_9000_crates = crates.clone();
    let crate_mover_9000 = CrateMover9000::new();
    for ins in &instructions {
        crate_mover_9000.execute(&mut crate_mover_9000_crates, ins);
    }

    let mut crate_mover_9001_crates = crates.clone();
    let crate_mover_9001 = CrateMover9001::new();
    for ins in &instructions {
        crate_mover_9001.execute(&mut crate_mover_9001_crates, ins);
    }

    println!(
        "Top crates using CraneMover9000: {}",
        crate_mover_9000_crates.top().join("")
    );
    println!(
        "Top crates using CraneMover9001: {}",
        crate_mover_9001_crates.top().join("")
    );
}
