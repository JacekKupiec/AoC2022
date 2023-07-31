use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use scanf::sscanf;

const COORDINATES_LOW_BOUND: i64 = 0;
const COORDINATES_HIGH_BOUND: i64 = 4_000_000;

fn manhattan_distance(x1 :i64, y1: i64, x2: i64, y2:i64) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

// merges vector of intervals that are sorted ascending by the left bound
fn merge(intervals: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    if intervals.is_empty() {
        return Vec::new();
    }

    let mut stack: Vec<_> = vec![intervals[0]];

    for interval in intervals.iter().skip(1) {
        let top = stack.pop().unwrap();

        // can be unified
        if top.0 <= interval.0 && interval.0 <= top.1 {
            let intervals_union = (top.0, max(top.1, interval.1));
            stack.push(intervals_union);
        } else { // intervals are disjoint
            stack.push(top);
            stack.push(*interval);
        }
    }

    return stack;
}

fn task1(sensors: &HashMap<(i64, i64), (i64, i64)>, beacons: &Vec<(i64, i64)>) -> usize {
    const POSITION_Y: i64 = 2_000_000;
    let mut intervals: Vec<_> = sensors.iter().filter_map(|(sensor, beacon)| {
        let distance = manhattan_distance(sensor.0, sensor.1, beacon.0, beacon.1);
        let distance_y = (sensor.1 - POSITION_Y).abs();
        let distance_x = distance - distance_y;

        if distance_x >= 0 {
            Some((sensor.0 - distance_x, sensor.0 + distance_x))
        } else {
            None
        }
    })
        .collect();

    intervals.sort_unstable_by(|i1, i2| i1.0.cmp(&i2.0));

    let merged_intervals = merge(intervals);
    let excluded_positions_count: i64 = merged_intervals.iter().map(|interval| interval.1 - interval.0 + 1).sum();
    let beacons_in_line = beacons.iter().filter(|b| b.1 == POSITION_Y).count();
    let result = excluded_positions_count as usize - beacons_in_line;

    return result;
}

fn exclude_positions(sensors: &HashMap<(i64, i64), (i64, i64)>, position_y: i64) -> Vec<(i64, i64)> {
    let mut intervals: Vec<_> = sensors.iter()
        .filter_map(|(sensor, beacon)| {
            let distance = manhattan_distance(sensor.0, sensor.1, beacon.0, beacon.1);
            let distance_y = (sensor.1 - position_y).abs();
            let distance_x = distance - distance_y;

            if distance_x < 0 {
                return None;
            }

            let interval = (sensor.0 - distance_x, sensor.0 + distance_x);

            if interval.0 > COORDINATES_HIGH_BOUND || interval.1 < COORDINATES_LOW_BOUND {
                return None;
            }

            return Some((max(COORDINATES_LOW_BOUND, interval.0), min(COORDINATES_HIGH_BOUND, interval.1)));
        })
        .collect();

    intervals.sort_unstable_by(|i1, i2| i1.0.cmp(&i2.0));

    return merge(intervals);
}

fn task2(sensors: &HashMap<(i64, i64), (i64, i64)>, coordinates_range: i64) -> i64 {
    const FREQUENCY_MULTIPLIER: i64 = 4_000_000;

    for position_y in 0..=coordinates_range {
        let intervals_of_excluded_positions = exclude_positions(sensors, position_y);

        if intervals_of_excluded_positions[0].0 == COORDINATES_LOW_BOUND + 1 {
            return COORDINATES_LOW_BOUND*FREQUENCY_MULTIPLIER + position_y;
        }

        if intervals_of_excluded_positions[0].1 == COORDINATES_LOW_BOUND - 1 {
            return COORDINATES_HIGH_BOUND*FREQUENCY_MULTIPLIER + position_y;
        }

        if intervals_of_excluded_positions.len() > 1 {
            return (intervals_of_excluded_positions[0].1 + 1)*FREQUENCY_MULTIPLIER + position_y;
        }
    }

    return -1;
}

fn main() {
    let path = "D:\\source\\Rust\\AoC 2022\\day_15\\src\\input.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut sensors: HashMap<(i64, i64), (i64, i64)> = HashMap::new();
    let mut beacons = Vec::new();

    for line in reader.lines().map(|l| l.unwrap() ) {
        let mut sensor_x = 0;
        let mut sensor_y = 0;
        let mut beacon_x = 0;
        let mut beacon_y = 0;

        let _ = sscanf!(line.as_str(), "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            sensor_x, sensor_y, beacon_x, beacon_y);

        let _ = sensors.insert((sensor_x, sensor_y), (beacon_x, beacon_y));
        let _ = beacons.push((beacon_x, beacon_y));
    }

    let result = task2(&sensors, COORDINATES_HIGH_BOUND);
    println!("Tuning frequency: {}", result);
}
