use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone)]
enum Arg {
    Constant(i64),
    Old,
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
struct SimpleExpression {
    arg1: Arg,
    arg2: Arg,
    op: Operation,
}

impl SimpleExpression {
    fn new(expr: String) -> Self {
        let expr_re = Regex::new(r"(old|\-?\d+) (\+|\-|\*|/) (old|\-?\d+)").unwrap();
        let caps = expr_re.captures(expr.as_str()).unwrap();
        let arg1_raw = caps.get(1).unwrap().as_str();
        let op_raw = caps.get(2).unwrap().as_str();
        let arg2_raw = caps.get(3).unwrap().as_str();
        Self {
            arg1: if arg1_raw == "old" {
                Arg::Old
            } else {
                Arg::Constant(arg1_raw.parse::<i64>().unwrap())
            },
            arg2: if arg2_raw == "old" {
                Arg::Old
            } else {
                Arg::Constant(arg2_raw.parse::<i64>().unwrap())
            },
            op: match op_raw {
                "+" => Operation::Add,
                "-" => Operation::Subtract,
                "*" => Operation::Multiply,
                "/" => Operation::Divide,
                _ => panic!("invalid operation"),
            },
        }
    }

    fn interp(&self, old: i64) -> i64 {
        let a1 = match self.arg1 {
            Arg::Constant(v) => v,
            Arg::Old => old,
        };
        let a2 = match self.arg2 {
            Arg::Constant(v) => v,
            Arg::Old => old,
        };
        match self.op {
            Operation::Add => a1 + a2,
            Operation::Subtract => a1 - a2,
            Operation::Multiply => a1 * a2,
            Operation::Divide => a1 / a2,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    worry_operation: SimpleExpression,
    test_divisor: i64,
    true_target_id: i64,
    false_target_id: i64,
}

#[derive(Debug)]
struct MonkeyGroup {
    monkey_ids: Vec<i64>,
    monkeys: HashMap<i64, Monkey>,
    inspections: HashMap<i64, u64>,
    is_relief: bool,
}

impl MonkeyGroup {
    fn parse(s: String, is_relief: bool) -> MonkeyGroup {
        let mut monkey_ids: Vec<i64> = Vec::new();
        let mut monkeys: HashMap<i64, Monkey> = HashMap::new();
        let monkey_re = Regex::new(r"Monkey (\d+):\n\s{2}Starting items: (\d+(,\s\d+)*)\n\s{2}Operation: new = (.+)\n\s{2}Test: divisible by (\d+)\n\s{4}If true: throw to monkey (\d+)\n\s{4}If false: throw to monkey (\d+)").unwrap();
        for m in monkey_re.find_iter(s.as_str()) {
            let caps = monkey_re.captures(m.as_str()).unwrap();
            let monkey_id = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let items = caps
                .get(2)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let worry_operation = caps.get(4).unwrap().as_str();
            let test_divisor = caps.get(5).unwrap().as_str().parse::<i64>().unwrap();
            let true_target = caps.get(6).unwrap().as_str().parse::<i64>().unwrap();
            let false_target = caps.get(7).unwrap().as_str().parse::<i64>().unwrap();
            let monkey = Monkey {
                items: items,
                worry_operation: SimpleExpression::new(String::from(worry_operation)),
                test_divisor: test_divisor,
                true_target_id: true_target,
                false_target_id: false_target,
            };
            monkeys.insert(monkey_id, monkey);
            monkey_ids.push(monkey_id);
        }
        MonkeyGroup {
            monkey_ids: monkey_ids,
            monkeys: monkeys,
            inspections: HashMap::new(),
            is_relief: is_relief,
        }
    }

    fn simulate_round(&mut self) {
        let divisor_mod = self
            .monkeys
            .values()
            .map(|v| v.test_divisor.clone())
            .reduce(|a, b| a * b)
            .unwrap();
        for monkey_id in &self.monkey_ids {
            let monkey = self.monkeys.get_mut(&monkey_id).unwrap();
            let items = monkey.items.clone();
            match self.inspections.get_mut(&monkey_id) {
                Some(v) => *v += items.len() as u64,
                None => {
                    self.inspections.insert(*monkey_id, items.len() as u64);
                }
            }
            monkey.items.clear();
            let monkey = self.monkeys.get(&monkey_id).unwrap().clone();
            for item in items {
                let item = if self.is_relief {
                    item
                } else {
                    match monkey.worry_operation.op {
                        Operation::Multiply => item % divisor_mod,
                        _ => item,
                    }
                };
                let item = monkey.worry_operation.interp(item);
                let item = if self.is_relief {
                    (item as f64 / 3.0).floor() as i64
                } else {
                    item
                };
                if item % monkey.test_divisor == 0 {
                    let target_monkey = self.monkeys.get_mut(&monkey.true_target_id).unwrap();
                    target_monkey.items.push(item);
                } else {
                    let target_monkey = self.monkeys.get_mut(&monkey.false_target_id).unwrap();
                    target_monkey.items.push(item);
                }
            }
        }
    }

    fn find_monkey_business(&self) -> u64 {
        let mut inspection_values = self
            .inspections
            .values()
            .map(|v| v.clone())
            .collect::<Vec<u64>>()
            .clone();
        inspection_values.sort_by(|a, b| b.cmp(&a));
        inspection_values[0] * inspection_values[1]
    }
}

fn main() {
    let stdin = io::stdin();
    let r = BufReader::new(stdin.lock());
    let mut s = String::new();
    for line in r.lines() {
        let l = line.unwrap();
        s.push_str(l.as_str());
        s.push('\n');
    }

    let mut monkey_group = MonkeyGroup::parse(s.clone(), true);
    for _ in 0..20 {
        monkey_group.simulate_round();
    }
    println!(
        "Monkey business with relief (20 rounds): {}",
        monkey_group.find_monkey_business()
    );

    let mut monkey_group = MonkeyGroup::parse(s.clone(), false);
    for _ in 0..10000 {
        monkey_group.simulate_round();
    }
    println!(
        "Monkey business with no relief (10000 rounds): {}",
        monkey_group.find_monkey_business()
    );
}
