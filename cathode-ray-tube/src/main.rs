use std::fmt;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone)]
enum Instruction {
    NoOp,
    AddX(i64),
}

struct CPU {
    pc: usize,
    cooldown: usize,
    x: i64,
    program: Vec<Instruction>,
}

impl CPU {
    fn new(program: Vec<Instruction>) -> CPU {
        CPU {
            pc: 0,
            cooldown: 0,
            x: 1,
            program: program,
        }
    }

    fn tick(&mut self) {
        let ins = &self.program[self.pc];
        if self.cooldown == 0 {
            match ins {
                Instruction::NoOp => self.pc += 1,
                Instruction::AddX(_) => {
                    self.cooldown = 2;
                }
            }
        } else if self.cooldown == 1 {
            match ins {
                Instruction::NoOp => {
                    panic!("invalid state");
                }
                Instruction::AddX(v) => {
                    self.x += v;
                    self.pc += 1;
                }
            }
        }
        if self.cooldown > 0 {
            self.cooldown -= 1;
        }
    }
}

struct CRT {
    cpu: CPU,
    position: (usize, usize),
    size: (usize, usize),
    buffer: Vec<Vec<char>>,
}

impl CRT {
    fn new(cpu: CPU) -> CRT {
        let max_r = 6;
        let max_c = 40;
        let buffer = (0..max_r)
            .map(|_| (0..max_c).map(|_| '.').collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        CRT {
            cpu: cpu,
            position: (0, 0),
            size: (max_r, max_c),
            buffer: buffer,
        }
    }

    fn tick(&mut self) {
        // Draw
        let (r, c) = self.position;
        if self.cpu.x == c as i64 || self.cpu.x - 1 == c as i64 || self.cpu.x + 1 == c as i64 {
            self.buffer[r][c] = '#';
        } else {
            self.buffer[r][c] = '.';
        }

        // Update CRT position
        let (mut r, mut c) = self.position;
        let (max_r, max_c) = self.size;
        c += 1;
        if c >= max_c {
            c = 0;
            r += 1;
            if r > max_r {
                r = 0;
            }
        }
        self.position = (r, c);

        // Update CPU
        self.cpu.tick()
    }
}

impl fmt::Display for CRT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (max_r, max_c) = self.size;
        let mut s = String::new();
        for r in 0..max_r {
            for c in 0..max_c {
                s.push(self.buffer[r][c]);
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

fn parse_instructions<R: BufRead>(r: BufReader<R>) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    for line in r.lines() {
        let l = line.unwrap();
        let args = l
            .split(" ")
            .map(|v| String::from(v))
            .collect::<Vec<String>>();
        match args[0].as_str() {
            "noop" => {
                program.push(Instruction::NoOp);
            }
            "addx" => {
                let v = args[1].parse::<i64>().unwrap();
                program.push(Instruction::AddX(v));
            }
            _ => {
                panic!("invalid operation");
            }
        }
    }
    program
}

fn find_total_interesting_signal_strength(cpu: &mut CPU) -> i64 {
    let mut total_signal_strength = 0;
    let interesting_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut target_cycles_idx = 0;
    for i in 0..*interesting_cycles.iter().max().unwrap() {
        let cycle = i + 1;
        if cycle == interesting_cycles[target_cycles_idx] {
            total_signal_strength += cycle * cpu.x;
            target_cycles_idx += 1;
        }
        cpu.tick();
    }
    total_signal_strength
}

fn main() {
    let stdin = io::stdin();
    let r = BufReader::new(stdin.lock());
    let instructions = parse_instructions(r);
    let mut cpu = CPU::new(instructions.clone());
    let total_interesting_signal_strength = find_total_interesting_signal_strength(&mut cpu);

    println!(
        "Total signal strength: {}",
        total_interesting_signal_strength
    );

    let cpu = CPU::new(instructions.clone());
    let mut crt = CRT::new(cpu);
    for _ in 0..240 {
        crt.tick();
    }
    println!("CRT output:\n{}", crt);
}
