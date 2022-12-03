use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, BufRead};

lazy_static! {
    static ref ROUND_RE: Regex = Regex::new(r"^(A|B|C)\s(X|Y|Z)$").unwrap();
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

#[derive(Debug, Copy, Clone)]
struct Round {
    shape1: Shape,
    shape2: Shape,
}

fn convert_opponent_code_to_shape(code: &str) -> Option<Shape> {
    if code == "A" {
        Some(Shape::Rock)
    } else if code == "B" {
        Some(Shape::Paper)
    } else if code == "C" {
        Some(Shape::Scissors)
    } else {
        None
    }
}

fn convert_player_code_to_shape(code: &str) -> Option<Shape> {
    if code == "X" {
        Some(Shape::Rock)
    } else if code == "Y" {
        Some(Shape::Paper)
    } else if code == "Z" {
        Some(Shape::Scissors)
    } else {
        None
    }
}

fn convert_player_code_to_outcome(code: &str) -> Option<Outcome> {
    if code == "X" {
        Some(Outcome::Lose)
    } else if code == "Y" {
        Some(Outcome::Draw)
    } else if code == "Z" {
        Some(Outcome::Win)
    } else {
        None
    }
}

fn find_player_outcome(round: &Round) -> Outcome {
    match round.shape1 {
        Shape::Rock => match round.shape2 {
            Shape::Rock => Outcome::Draw,
            Shape::Paper => Outcome::Win,
            Shape::Scissors => Outcome::Lose,
        },
        Shape::Paper => match round.shape2 {
            Shape::Rock => Outcome::Lose,
            Shape::Paper => Outcome::Draw,
            Shape::Scissors => Outcome::Win,
        },
        Shape::Scissors => match round.shape2 {
            Shape::Rock => Outcome::Win,
            Shape::Paper => Outcome::Lose,
            Shape::Scissors => Outcome::Draw,
        },
    }
}

fn find_player_shape(opponent_shape: Shape, outcome: Outcome) -> Shape {
    match opponent_shape {
        Shape::Rock => match outcome {
            Outcome::Lose => Shape::Scissors,
            Outcome::Draw => Shape::Rock,
            Outcome::Win => Shape::Paper,
        },
        Shape::Paper => match outcome {
            Outcome::Lose => Shape::Rock,
            Outcome::Draw => Shape::Paper,
            Outcome::Win => Shape::Scissors,
        },
        Shape::Scissors => match outcome {
            Outcome::Lose => Shape::Paper,
            Outcome::Draw => Shape::Scissors,
            Outcome::Win => Shape::Rock,
        },
    }
}

fn find_player_score(rounds: &Vec<Round>) -> i64 {
    return rounds
        .iter()
        .map(|round| {
            let score1 = match round.shape2 {
                Shape::Rock => 1,
                Shape::Paper => 2,
                Shape::Scissors => 3,
            };
            let outcome = find_player_outcome(round);
            let score2 = match outcome {
                Outcome::Win => 6,
                Outcome::Draw => 3,
                Outcome::Lose => 0,
            };
            score1 + score2
        })
        .sum();
}

fn parse_round_by_shape(l: &String) -> Round {
    let caps = ROUND_RE.captures(l.as_str()).unwrap();
    let code1 = caps.get(1).unwrap().as_str();
    let code2 = caps.get(2).unwrap().as_str();
    let shape1 = convert_opponent_code_to_shape(code1).unwrap();
    let shape2 = convert_player_code_to_shape(code2).unwrap();
    return Round {
        shape1: shape1,
        shape2: shape2,
    };
}

fn parse_round_by_outcome(l: &String) -> Round {
    let caps = ROUND_RE.captures(l.as_str()).unwrap();
    let code1 = caps.get(1).unwrap().as_str();
    let code2 = caps.get(2).unwrap().as_str();
    let shape1 = convert_opponent_code_to_shape(code1).unwrap();
    let outcome = convert_player_code_to_outcome(code2).unwrap();
    let shape2 = find_player_shape(shape1, outcome);
    return Round {
        shape1: shape1,
        shape2: shape2,
    };
}

fn main() {
    let stdin = io::stdin();
    let mut rounds_by_shape = vec![];
    let mut rounds_by_outcome = vec![];
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        rounds_by_shape.push(parse_round_by_shape(&l));
        rounds_by_outcome.push(parse_round_by_outcome(&l));
    }

    println!(
        "Total player score assuming shape codes: {}",
        find_player_score(&rounds_by_shape)
    );
    println!(
        "Total player score assuming outcome codes: {}",
        find_player_score(&rounds_by_outcome)
    );
}
