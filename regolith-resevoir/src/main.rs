use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Cave {
    coord_rect_bounds: (i64, i64, i64, i64),
    data: Vec<Vec<char>>,
}

impl Cave {
    fn parse_rock_formation(s: String) -> Vec<(i64, i64)> {
        let mut pts = Vec::new();
        for pt_s in s.split(" -> ") {
            let coords = pt_s
                .split(",")
                .map(|v| v.parse().unwrap())
                .collect::<Vec<i64>>();
            pts.push((coords[0], coords[1]));
        }
        pts
    }

    fn find_coord_rect_bounds(rocks: &Vec<Vec<(i64, i64)>>) -> (i64, i64, i64, i64) {
        let mut all_pts = Vec::new();
        for pts in rocks {
            for pt in pts {
                all_pts.push(*pt);
            }
        }
        all_pts.sort_by(|&pt1, &pt2| {
            let (r1, _) = pt1;
            let (r2, _) = pt2;
            r1.cmp(&r2)
        });
        let (min_x, _) = *all_pts.first().unwrap();
        let (max_x, _) = *all_pts.last().unwrap();
        all_pts.sort_by(|&pt1, &pt2| {
            let (_, c1) = pt1;
            let (_, c2) = pt2;
            c1.cmp(&c2)
        });
        let (_, max_y) = *all_pts.last().unwrap();
        (min_x, max_x, 0, max_y)
    }

    fn new(rocks: &Vec<Vec<(i64, i64)>>, has_floor: bool) -> Self {
        let coord_rect_bounds = if has_floor {
            let pre_coord_rect_bounds = Self::find_coord_rect_bounds(rocks);
            let (min_x, max_x, min_y, max_y) = pre_coord_rect_bounds;
            let dist = max_y + 2 - min_y;
            (min_x - dist, max_x + dist, min_y, max_y + 2)
        } else {
            Self::find_coord_rect_bounds(rocks)
        };
        let (min_x, max_x, min_y, max_y) = coord_rect_bounds;

        let mut data = Vec::new();
        for _ in min_y..max_y + 1 {
            let mut row = Vec::new();
            for _ in min_x..max_x + 1 {
                row.push('.');
            }
            data.push(row);
        }
        if has_floor {
            let extra_cs = vec!['.', '#'];
            for (i, &c) in extra_cs.iter().enumerate() {
                let mut row = Vec::new();
                for _ in min_x..max_x + 1 {
                    row.push(c);
                }
                let idx = data.len() - extra_cs.len() + i;
                data[idx] = row;
            }
        }

        let mut cave = Self {
            coord_rect_bounds: coord_rect_bounds,
            data: data,
        };
        for pts in rocks {
            for i in 1..pts.len() {
                let curr_pt = pts[i - 1];
                let dest_pt = pts[i];
                let mut curr = cave.coords2idxs(curr_pt);
                let dest = cave.coords2idxs(dest_pt);
                while curr != dest {
                    let (curr_r, curr_c) = curr;
                    let (dest_r, dest_c) = dest;

                    cave.data[curr_r][curr_c] = '#';

                    if curr_r == dest_r {
                        if curr_c < dest_c {
                            curr = (curr_r, curr_c + 1);
                        } else if curr_c > dest_c {
                            curr = (curr_r, curr_c - 1);
                        }
                    } else if curr_c == dest_c {
                        if curr_r < dest_r {
                            curr = (curr_r + 1, curr_c);
                        } else if curr_r > dest_r {
                            curr = (curr_r - 1, curr_c);
                        }
                    }
                }

                let (dest_r, dest_c) = dest;
                cave.data[dest_r][dest_c] = '#';
            }
        }
        cave
    }

    fn simulate_sand(&mut self) -> u64 {
        let mut counter = 0;
        loop {
            let res = self.drop_sand();
            match res {
                Ok(()) => {
                    counter += 1;
                }
                Err(()) => {
                    break;
                }
            }
        }
        counter
    }

    fn drop_sand(&mut self) -> Result<(), ()> {
        let max_r = self.data.len();
        let max_c = self.data[0].len();
        let curr_pt = (500, 0);
        let mut curr = self.coords2idxs(curr_pt);
        loop {
            let (curr_r, curr_c) = curr;
            if self.data[curr_r][curr_c] != '.' {
                return Err(());
            }
            let offsets = vec![(1, 0), (1, -1), (1, 1)];
            let mut is_offset_found = false;
            for (dr, dc) in offsets {
                let next_r = curr_r as i64 + dr;
                let next_c = curr_c as i64 + dc;
                if next_r >= 0 && next_r < max_r as i64 && next_c >= 0 && next_c < max_c as i64 {
                    let next_r = next_r as usize;
                    let next_c = next_c as usize;
                    if self.data[next_r][next_c] == '.' {
                        curr = (next_r, next_c);
                        is_offset_found = true;
                        break;
                    }
                } else {
                    return Err(());
                }
            }
            if !is_offset_found {
                break;
            }
        }
        let (curr_r, curr_c) = curr;
        self.data[curr_r][curr_c] = '0';
        Ok(())
    }

    fn coords2idxs(&self, coords: (i64, i64)) -> (usize, usize) {
        let (x, y) = coords;
        let (min_x, _, min_y, _) = self.coord_rect_bounds;
        let c = x - min_x;
        let r = y - min_y;
        (r as usize, c as usize)
    }
}

fn main() {
    let stdin = io::stdin();
    let r = BufReader::new(stdin.lock());

    let mut rocks = Vec::new();
    for line in r.lines() {
        let l = line.unwrap();
        rocks.push(Cave::parse_rock_formation(l));
    }

    let mut cave_without_floor = Cave::new(&rocks, false);
    let counter = cave_without_floor.simulate_sand();
    println!("Units of sand until fall off: {}", counter);

    let mut cave_with_floor = Cave::new(&rocks, true);
    let counter = cave_with_floor.simulate_sand();
    println!("Units of sand until blocked: {}", counter);
}
