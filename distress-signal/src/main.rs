use std::cell::RefCell;
use std::cmp::Ordering;
use std::io::{self, BufRead, BufReader};
use std::rc::Rc;

#[derive(Debug, Clone)]
enum Packet<T> {
    List(Vec<Rc<RefCell<Packet<T>>>>),
    Atom(T),
}

impl PartialOrd for Packet<i64> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let diff = Self::compare_packets(self, other);
        Some(if diff < 0 {
            std::cmp::Ordering::Less
        } else if diff == 0 {
            std::cmp::Ordering::Equal
        } else {
            std::cmp::Ordering::Greater
        })
    }
}

impl PartialEq for Packet<i64> {
    fn eq(&self, other: &Self) -> bool {
        Self::compare_packets(self, other) == 0
    }
}

impl Packet<i64> {
    fn compare_packets(p1: &Packet<i64>, p2: &Packet<i64>) -> i64 {
        match (p1, p2) {
            (Packet::Atom(v1), Packet::Atom(v2)) => v1 - v2,
            (Packet::Atom(v1), Packet::List(l2)) => {
                let l1 = vec![Rc::new(RefCell::new(Packet::Atom(*v1)))];
                Self::compare_packet_list_values(&l1, l2)
            }
            (Packet::List(l1), Packet::Atom(v2)) => {
                let l2 = vec![Rc::new(RefCell::new(Packet::Atom(*v2)))];
                Self::compare_packet_list_values(l1, &l2)
            }
            (Packet::List(l1), Packet::List(l2)) => Self::compare_packet_list_values(l1, l2),
        }
    }

    fn compare_packet_list_values(
        l1: &Vec<Rc<RefCell<Packet<i64>>>>,
        l2: &Vec<Rc<RefCell<Packet<i64>>>>,
    ) -> i64 {
        let mut i = 0;
        while i < l1.len() && i < l2.len() {
            let p1 = l1[i].borrow();
            let p2 = l2[i].borrow();
            let res = Self::compare_packets(&p1, &p2);
            if res != 0 {
                return res;
            } else {
                i += 1
            }
        }
        l1.len() as i64 - l2.len() as i64
    }
}

fn parse_packet_pairs<R: BufRead>(r: BufReader<R>) -> Vec<(Packet<i64>, Packet<i64>)> {
    let mut packet_pairs: Vec<(Packet<i64>, Packet<i64>)> = Vec::new();
    let mut lines_buffer: Vec<String> = Vec::new();
    for line in r.lines() {
        let l = line.unwrap();
        if l.len() == 0 {
            let res1 = parse_packet(&lines_buffer[0].chars().collect(), 0);
            let res2 = parse_packet(&lines_buffer[1].chars().collect(), 0);
            match (res1, res2) {
                (Ok((p1, _)), Ok((p2, _))) => {
                    packet_pairs.push((p1, p2));
                }
                _ => panic!("invalid input"),
            }
            lines_buffer.clear();
        } else {
            lines_buffer.push(l);
        }
    }
    let res1 = parse_packet(&lines_buffer[0].chars().collect(), 0);
    let res2 = parse_packet(&lines_buffer[1].chars().collect(), 0);
    match (res1, res2) {
        (Ok((p1, _)), Ok((p2, _))) => {
            packet_pairs.push((p1, p2));
        }
        _ => panic!("invalid input"),
    }
    packet_pairs
}

fn parse_packet(char_buffer: &Vec<char>, idx: usize) -> Result<(Packet<i64>, usize), ()> {
    match char_buffer[idx] {
        '[' => parse_packet_list(char_buffer, idx),
        '-' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
            parse_packet_atom(char_buffer, idx)
        }
        _ => Err(()),
    }
}

fn parse_packet_atom(char_buffer: &Vec<char>, idx: usize) -> Result<(Packet<i64>, usize), ()> {
    let mut curr_idx = idx;
    let mut digit_buffer = String::new();
    loop {
        match char_buffer[curr_idx] {
            '-' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                digit_buffer.push(char_buffer[curr_idx]);
                curr_idx += 1;
            }
            _ => {
                let res = digit_buffer.parse::<i64>();
                match res {
                    Ok(v) => {
                        let p = Packet::Atom(v);
                        return Ok((p, curr_idx));
                    }
                    _ => return Err(()),
                }
            }
        }
    }
}

fn parse_packet_list(char_buffer: &Vec<char>, idx: usize) -> Result<(Packet<i64>, usize), ()> {
    let mut data = Vec::new();
    let mut curr_idx = idx;
    match char_buffer[curr_idx] {
        '[' => {
            curr_idx += 1;
            loop {
                match char_buffer[curr_idx] {
                    ']' => {
                        let mut l = Vec::new();
                        for p in data {
                            let p_rc = Rc::new(RefCell::new(p));
                            l.push(p_rc);
                        }
                        let p = Packet::List(l);
                        return Ok((p, curr_idx + 1));
                    }
                    _ => {
                        let res = parse_packet(char_buffer, curr_idx);
                        match res {
                            Ok((p, idx)) => {
                                data.push(p);
                                match char_buffer[idx] {
                                    ',' => {
                                        curr_idx = idx + 1;
                                    }
                                    ']' => {
                                        curr_idx = idx;
                                    }
                                    _ => return Err(()),
                                }
                            }
                            _ => return Err(()),
                        }
                    }
                }
            }
        }
        _ => Err(()),
    }
}

fn main() {
    let stdin = io::stdin();
    let r = BufReader::new(stdin.lock());
    let packet_pairs = parse_packet_pairs(r);

    let mut s = 0;
    for (i, packet_pair) in packet_pairs.iter().enumerate() {
        let (p1, p2) = packet_pair;
        if p1 < p2 {
            s += i + 1;
        }
    }
    println!("Ordered packet index sum: {}", s);

    let divider_p1 = Packet::List(vec![Rc::new(RefCell::new(Packet::List(vec![Rc::new(
        RefCell::new(Packet::Atom(2)),
    )])))]);
    let divider_p2 = Packet::List(vec![Rc::new(RefCell::new(Packet::List(vec![Rc::new(
        RefCell::new(Packet::Atom(6)),
    )])))]);
    let mut all_packets = Vec::new();
    for packet_pair in packet_pairs {
        let (p1, p2) = packet_pair;
        all_packets.push(p1);
        all_packets.push(p2);
    }
    all_packets.push(divider_p1.clone());
    all_packets.push(divider_p2.clone());
    all_packets.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut m = 1;
    for (i, packet) in all_packets.iter().enumerate() {
        if divider_p1 == *packet || divider_p2 == *packet {
            m *= i + 1
        }
    }
    println!("Distress signal decoder key: {}", m);
}
