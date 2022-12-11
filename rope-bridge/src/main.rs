use std::collections::HashMap;
use std::fmt;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    count: u64,
}

fn parse_commands<R: BufRead>(r: BufReader<R>) -> Vec<Command> {
    let mut cmds: Vec<Command> = Vec::new();
    for line in r.lines() {
        let l = line.unwrap();
        let args = l
            .split(" ")
            .map(|v| String::from(v))
            .collect::<Vec<String>>();
        let direction = match args[0].as_str() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("invalid direction"),
        };
        let count = args[1].parse::<u64>().unwrap();
        cmds.push(Command {
            direction: direction,
            count: count,
        })
    }
    cmds
}

struct Simulation {
    knots: Vec<(i64, i64)>,
    rect_bounds: (i64, i64, i64, i64),
    tail_locations: HashMap<(i64, i64), u64>,
}

impl Simulation {
    fn new(n: usize) -> Simulation {
        Simulation {
            knots: (0..n).map(|_| (0, 0)).collect(),
            rect_bounds: (-10, -10, 10, 10),
            tail_locations: HashMap::new(),
        }
    }

    fn execute_command(&mut self, cmd: &Command) {
        let (dr, dc) = match cmd.direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };
        for _ in 0..cmd.count {
            // Move head
            let (curr_knot_r, curr_knot_c) = self.knots[0];
            self.knots[0] = (curr_knot_r + dr, curr_knot_c + dc);
            // Move each knot
            for i in 1..self.knots.len() {
                // Move knot if outside of box
                let (prev_knot_r, prev_knot_c) = self.knots[i - 1];
                let (curr_knot_r, curr_knot_c) = self.knots[i];
                if i64::pow(prev_knot_r - curr_knot_r, 2) + i64::pow(prev_knot_c - curr_knot_c, 2)
                    > 2
                {
                    let new_curr_knot_r = if prev_knot_r > curr_knot_r {
                        curr_knot_r + 1
                    } else if prev_knot_r < curr_knot_r {
                        curr_knot_r - 1
                    } else {
                        curr_knot_r
                    };
                    let new_curr_knot_c = if prev_knot_c > curr_knot_c {
                        curr_knot_c + 1
                    } else if prev_knot_c < curr_knot_c {
                        curr_knot_c - 1
                    } else {
                        curr_knot_c
                    };
                    self.knots[i] = (new_curr_knot_r, new_curr_knot_c);
                }
            }

            let tail = self.knots[self.knots.len() - 1];
            match self.tail_locations.get_mut(&tail) {
                Some(v) => *v += 1,
                None => {
                    self.tail_locations.insert(tail, 1);
                }
            }
        }
    }
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (min_r, min_c, max_r, max_c) = self.rect_bounds;
        let mut s = String::new();
        for r in min_r..max_r {
            for c in min_c..max_c {
                let mut was_found = false;
                for i in 0..self.knots.len() {
                    if (r, c) == self.knots[i] {
                        was_found = true;
                        if i == 0 {
                            s.push('H');
                        } else {
                            s.push_str(&i.to_string());
                        }
                        break;
                    }
                }
                if !was_found {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

fn main() {
    let stdin = io::stdin();
    let r = BufReader::new(stdin.lock());
    let cmds = parse_commands(r);
    let mut two_knot_sim = Simulation::new(2);
    for cmd in &cmds {
        two_knot_sim.execute_command(&cmd);
    }
    println!("Number of locations visited by tail (2 knots): {}", two_knot_sim.tail_locations.len());

    let mut ten_knot_sim = Simulation::new(10);
    for cmd in &cmds {
        ten_knot_sim.execute_command(&cmd);
    }
    println!("Number of locations visited by tail (10 knots): {}", ten_knot_sim.tail_locations.len());
}
