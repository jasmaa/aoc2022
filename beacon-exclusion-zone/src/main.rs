use regex::Regex;
use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader},
};

#[derive(Debug)]
struct Sensor {
    location: (i64, i64),
    closest_beacon_location: (i64, i64),
}

impl Sensor {
    fn parse(s: String) -> Sensor {
        let sensor_re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();
        let cap = sensor_re.captures(s.as_str()).unwrap();
        let location_x = cap[1].parse::<i64>().unwrap();
        let location_y = cap[2].parse::<i64>().unwrap();
        let closest_beacon_location_x = cap[3].parse::<i64>().unwrap();
        let closest_beacon_location_y = cap[4].parse::<i64>().unwrap();
        Sensor {
            location: (location_x, location_y),
            closest_beacon_location: (closest_beacon_location_x, closest_beacon_location_y),
        }
    }

    fn find_x_range(&self, y: i64) -> (i64, i64) {
        let (location_x, location_y) = self.location;
        let closest_beacon_dist = find_manhattan_dist(self.location, self.closest_beacon_location);
        let y_dist = (location_y - y).abs();
        let remaining_dist = closest_beacon_dist - y_dist;
        let min_x = location_x - remaining_dist;
        let max_x = location_x + remaining_dist;
        (min_x, max_x)
    }
}

fn find_manhattan_dist(a: (i64, i64), b: (i64, i64)) -> i64 {
    let (a_x, a_y) = a;
    let (b_x, b_y) = b;
    (a_x - b_x).abs() + (a_y - b_y).abs()
}

fn find_non_overlapping_x_ranges(sensors: &Vec<Sensor>, y: i64) -> Vec<(i64, i64)> {
    let mut x_ranges = sensors
        .iter()
        .map(|v| v.find_x_range(y))
        .filter(|&v| {
            let (start, end) = v;
            start <= end
        })
        .collect::<Vec<(i64, i64)>>();
    x_ranges.sort();

    let mut non_overlapping_x_ranges = Vec::new();
    if x_ranges.len() == 0 {
        return non_overlapping_x_ranges;
    }
    let mut curr_x_range = x_ranges[0];
    for i in 1..x_ranges.len() {
        let (a_start, a_end) = curr_x_range;
        let (b_start, b_end) = x_ranges[i];
        if b_start <= a_end + 1 {
            curr_x_range = (a_start, a_end.max(b_end));
        } else {
            non_overlapping_x_ranges.push(curr_x_range);
            curr_x_range = x_ranges[i];
        }
    }
    non_overlapping_x_ranges.push(curr_x_range);
    non_overlapping_x_ranges
}

fn find_total_non_beacon_locations(sensors: &Vec<Sensor>, y: i64) -> i64 {
    let beacon_locations = sensors
        .iter()
        .map(|v| v.closest_beacon_location)
        .collect::<HashSet<(i64, i64)>>();

    let non_overlapping_x_ranges = find_non_overlapping_x_ranges(sensors, y);

    let mut beacons_in_x_ranges = 0;
    for (beacon_x, beacon_y) in &beacon_locations {
        for (x_min, x_max) in &non_overlapping_x_ranges {
            if *beacon_y == y && *x_min <= *beacon_x && *x_max >= *beacon_x {
                beacons_in_x_ranges += 1;
                break;
            }
        }
    }

    let mut counter = 0;
    for (x_min, x_max) in non_overlapping_x_ranges {
        counter += x_max - x_min + 1;
    }
    counter -= beacons_in_x_ranges;
    counter
}

fn find_distress_beacon_location(sensors: &Vec<Sensor>, y_range: (i64, i64)) -> Option<(i64, i64)> {
    let (min_y, max_y) = y_range;
    for y in min_y..(max_y + 1) {
        let non_overlapping_x_ranges = find_non_overlapping_x_ranges(sensors, y);
        if non_overlapping_x_ranges.len() > 1 {
            let (_, end_x) = non_overlapping_x_ranges[0];
            return Some((end_x + 1, y));
        }
    }
    None
}

fn calculate_tuning_frequency(location: (i64, i64)) -> i64 {
    let (x, y) = location;
    4000000 * x + y
}

fn main() {
    let stdin = io::stdin();
    let r = BufReader::new(stdin.lock());

    let mut sensors = Vec::new();
    for line in r.lines() {
        let l = line.unwrap();
        let sensor = Sensor::parse(l);
        sensors.push(sensor);
    }

    println!(
        "Total non-beacon locations: {}",
        find_total_non_beacon_locations(&sensors, 2000000)
    );

    println!(
        "Distress beacon tuning frequency: {}",
        calculate_tuning_frequency(find_distress_beacon_location(&sensors, (0, 4000000)).unwrap())
    );
}
