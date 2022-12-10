use std::collections::HashMap;
use std::io::{self, BufRead};

fn find_start_index(buffer: &String, unique_char_count: usize) -> Option<usize> {
    let mut h = HashMap::new();
    let chars = buffer.as_str().chars();
    for (idx, c) in chars.enumerate() {
        match h.get_mut(&c) {
            Some(v) => {
                *v += 1;
            }
            None => {
                h.insert(c, 1);
            }
        }
        if idx >= unique_char_count {
            let last_c = buffer.chars().nth(idx - unique_char_count).unwrap();
            let v = h.get_mut(&last_c).unwrap();
            if *v == 1 {
                h.remove(&last_c);
            } else {
                *v -= 1;
            }
        }
        if h.iter().count() == unique_char_count
            && h.iter().fold(true, |acc, (_, v)| acc && *v == 1)
        {
            return Some(idx);
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_line(&mut buffer).unwrap();
    println!(
        "Start of packet marker: {}",
        find_start_index(&buffer, 4).unwrap() + 1
    );
    println!(
        "Start of message marker: {}",
        find_start_index(&buffer, 14).unwrap() + 1
    );
}
