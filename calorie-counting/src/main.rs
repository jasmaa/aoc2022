use std::io::{self, BufRead};

fn find_max_calorie_elf(items: &Vec<Vec<i64>>) -> i64 {
    let v: Vec<i64> = items
        .iter()
        .map(|subsection| subsection.iter().sum())
        .collect();
    *v.iter().max().unwrap()
}

fn find_top_k_calorie_elves(items: &Vec<Vec<i64>>, k: usize) -> i64 {
    let mut v: Vec<i64> = items
        .iter()
        .map(|subsection| subsection.iter().sum())
        .collect();
    v.sort_by(|a, b| b.cmp(a));
    v[0..k].iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let mut items = vec![vec![]];
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.len() == 0 {
            items.push(vec![]);
        } else {
            let idx = items.len() - 1;
            items[idx].push(l.parse::<i64>().unwrap());
        }
    }

    println!("Max calorie elf: {}", find_max_calorie_elf(&items));
    println!(
        "Sum of calories of top 3 elves: {}",
        find_top_k_calorie_elves(&items, 3)
    );
}
