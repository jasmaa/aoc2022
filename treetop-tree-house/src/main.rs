use std::io::{self, BufRead, BufReader};

struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<Vec<T>>,
}

impl Grid<u64> {
    fn parse<R: BufRead>(r: BufReader<R>) -> Self {
        let data = r
            .lines()
            .map(|line| {
                let l = line.unwrap();
                l.chars()
                    .map(|c| String::from(c).parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>();
        let h = data.len();
        let w = data[0].len();
        Grid {
            height: h,
            width: w,
            data: data,
        }
    }
}

fn is_visible(grid: &Grid<u64>, r: usize, c: usize) -> bool {
    let steps = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    for step in steps {
        let (dr, dc) = step;
        let mut curr_r = r as i64;
        let mut curr_c = c as i64;
        let mut visible = true;
        loop {
            curr_r += dr;
            curr_c += dc;
            if curr_r < 0
                || curr_c < 0
                || curr_r >= grid.height as i64
                || curr_c >= grid.width as i64
            {
                break;
            }
            if grid.data[curr_r as usize][curr_c as usize] >= grid.data[r][c] {
                visible = false;
                break;
            }
        }
        if visible {
            return true;
        }
    }
    false
}

fn find_scenic_score(grid: &Grid<u64>, r: usize, c: usize) -> u64 {
    let mut scenic_score = 1;
    let steps = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    for step in steps {
        let (dr, dc) = step;
        let mut curr_r = r as i64;
        let mut curr_c = c as i64;
        let mut dist = 0;
        loop {
            curr_r += dr;
            curr_c += dc;
            if curr_r < 0
                || curr_c < 0
                || curr_r >= grid.height as i64
                || curr_c >= grid.width as i64
            {
                break;
            }
            dist += 1;
            if grid.data[curr_r as usize][curr_c as usize] >= grid.data[r][c] {
                break;
            }
        }
        scenic_score *= dist;
    }
    scenic_score
}

fn count_visible_trees(grid: &Grid<u64>) -> u64 {
    let mut count = 0;
    for i in 0..grid.height {
        for j in 0..grid.width {
            if is_visible(grid, i, j) {
                count += 1;
            }
        }
    }
    count
}

fn find_max_scenic_score(grid: &Grid<u64>) -> u64 {
    let mut best_scenic_score = 0;
    for i in 0..grid.height {
        for j in 0..grid.width {
            let scenic_score = find_scenic_score(grid, i, j);
            if scenic_score > best_scenic_score {
                best_scenic_score = scenic_score;
            }
        }
    }
    best_scenic_score
}

fn main() {
    let stdin = io::stdin();
    let r = BufReader::new(stdin.lock());
    let grid = Grid::parse(r);
    println!("Total visible trees: {}", count_visible_trees(&grid));
    println!("Max scenic score for a tree: {}", find_max_scenic_score(&grid));
}
