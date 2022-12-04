use regex::Regex;
use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Assignment {
    ranges: Vec<(i64, i64)>,
}

impl Assignment {
    fn new(l: &String) -> Assignment {
        let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
        let caps = re.captures(l.as_str()).unwrap();
        let r1_start = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let r1_end = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let r2_start = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let r2_end = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
        let mut v: Vec<(i64, i64)> = vec![(r1_start, r1_end), (r2_start, r2_end)];
        v.sort_by(|a, b| {
            let (r1_start, r1_end) = a;
            let (r2_start, r2_end) = b;
            let c1 = r1_start.cmp(r2_start);
            match c1 {
                Ordering::Equal => r2_end.cmp(r1_end),
                _ => c1,
            }
        });
        Assignment { ranges: v }
    }

    fn num_fully_contained_ranges(&self) -> u64 {
        let mut counter = 0;
        for i in 0..self.ranges.len() {
            let r1 = self.ranges[i];
            for j in (i + 1)..self.ranges.len() {
                let r2 = self.ranges[j];
                let (_, r1_end) = r1;
                let (_, r2_end) = r2;
                if r2_end <= r1_end {
                    counter += 1
                }
            }
        }
        counter
    }

    fn num_overlapping_ranges(&self) -> u64 {
        let mut counter = 0;
        for i in 0..self.ranges.len() {
            let r1 = self.ranges[i];
            for j in (i + 1)..self.ranges.len() {
                let r2 = self.ranges[j];
                let (_, r1_end) = r1;
                let (r2_start, _) = r2;
                if r2_start <= r1_end {
                    counter += 1
                }
            }
        }
        counter
    }
}

fn find_total_assignments_fully_contained_ranges(assignments: &Vec<Assignment>) -> u64 {
    assignments
        .iter()
        .map(|a| a.num_fully_contained_ranges())
        .sum()
}

fn find_total_assignments_overlapping_ranges(assignments: &Vec<Assignment>) -> u64 {
    assignments.iter().map(|a| a.num_overlapping_ranges()).sum()
}

fn main() {
    let stdin = io::stdin();
    let mut assignments: Vec<Assignment> = Vec::new();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let assignment = Assignment::new(&l);
        assignments.push(assignment);
    }

    println!(
        "Total assignments with fully contained ranges: {}",
        find_total_assignments_fully_contained_ranges(&assignments)
    );
    println!(
        "Total assignments with overlapping ranges: {}",
        find_total_assignments_overlapping_ranges(&assignments)
    );
}
