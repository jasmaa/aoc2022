use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<u64>>,
    size: (usize, usize),
    start: (usize, usize),
    end: (usize, usize),
}

impl Grid {
    fn parse<R: BufRead>(r: BufReader<R>) -> Self {
        let mut data = Vec::new();
        let mut start_opt = None;
        let mut end_opt = None;
        for (i, line) in r.lines().enumerate() {
            let l = line.unwrap();
            let mut row = Vec::new();
            for (j, c) in l.chars().enumerate() {
                if c == 'S' {
                    start_opt = Some((i, j));
                } else if c == 'E' {
                    end_opt = Some((i, j));
                }
                let height = if c == 'S' {
                    0
                } else if c == 'E' {
                    25
                } else {
                    c as u64 - 'a' as u64
                };
                row.push(height);
            }
            data.push(row)
        }

        let h = data.len();
        let w = data[0].len();

        Self {
            data: data,
            size: (h, w),
            start: start_opt.unwrap(),
            end: end_opt.unwrap(),
        }
    }

    fn find_shortest_steps_from_start(&self) -> Option<u64> {
        let steps_cache = self.build_steps_cache(self.start);
        let (end_i, end_j) = self.end;
        steps_cache[end_i][end_j]
    }

    fn find_shortest_steps_from_any_lowest_point(&self) -> Option<u64> {
        let (h, w) = self.size;
        let (start_i, start_j) = self.start;
        let (end_i, end_j) = self.end;
        let mut starts = Vec::new();
        for i in 0..h {
            for j in 0..w {
                if self.data[i][j] == 0 {
                    if i != start_i || j != start_j {
                        starts.push((i, j));
                    }
                }
            }
        }
        let step_opts = starts
            .iter()
            .map(|&start| {
                let steps_cache = self.build_steps_cache(start);
                steps_cache[end_i][end_j]
            })
            .collect::<Vec<Option<u64>>>();
        if step_opts.len() > 0 {
            let best_step_opt = step_opts
                .iter()
                .filter(|&&v| match v {
                    Some(_) => true,
                    None => false,
                })
                .map(|v| v.unwrap())
                .min();
            Some(best_step_opt.unwrap())
        } else {
            None
        }
    }

    fn build_steps_cache(&self, start: (usize, usize)) -> Vec<Vec<Option<u64>>> {
        let (h, w) = self.size;
        let mut steps_cache = (0..h)
            .map(|_| (0..w).map(|_| None).collect::<Vec<Option<u64>>>())
            .collect::<Vec<Vec<Option<u64>>>>();

        let (start_i, start_j) = start;
        let mut frontier = vec![start];
        steps_cache[start_i][start_j] = Some(0);

        while frontier.len() > 0 {
            let mut buffer: Vec<(usize, usize)> = Vec::new();
            for (i, j) in frontier {
                let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                let valid_offsets = offsets
                    .iter()
                    .filter(|(di, dj)| {
                        let h = h as i64;
                        let w = w as i64;
                        let next_i = i as i64 + di;
                        let next_j = j as i64 + dj;
                        next_i >= 0 && next_i < h && next_j >= 0 && next_j < w
                    })
                    .map(|v| *v)
                    .collect::<Vec<(i64, i64)>>();
                for (di, dj) in valid_offsets.iter() {
                    let next_i = (i as i64 + di) as usize;
                    let next_j = (j as i64 + dj) as usize;
                    if self.data[next_i][next_j] <= self.data[i][j]
                        || self.data[next_i][next_j] == self.data[i][j] + 1
                    {
                        match steps_cache[next_i][next_j] {
                            Some(_) => {}
                            None => {
                                let step = steps_cache[i][j].unwrap();
                                steps_cache[next_i][next_j] = Some(step + 1);
                                buffer.push((next_i, next_j));
                            }
                        }
                    }
                }
            }
            frontier = buffer;
        }
        steps_cache
    }
}

fn main() {
    let stdin = io::stdin();
    let r = BufReader::new(stdin.lock());
    let grid = Grid::parse(r);

    println!(
        "Shortest steps from start to end: {}",
        grid.find_shortest_steps_from_start().unwrap()
    );
    println!(
        "Shortest steps from any lowest point to end: {}",
        grid.find_shortest_steps_from_any_lowest_point().unwrap()
    );
}
