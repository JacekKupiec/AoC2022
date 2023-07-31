use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use scanf::sscanf;

fn manhattan_distance(x1 :i32, y1: i32, x2: i32, y2:i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

// merges vector of intervals that are sorted ascending by the left bound
fn merge(intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
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

fn main() {
    let path = "D:\\source\\Rust\\AoC 2022\\day_15\\src\\input.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    const POSITION_Y: i32 = 2_000_000;
    let mut sensors: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
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
    let excluded_positions_count: i32 = merged_intervals.iter().map(|interval| interval.1 - interval.0 + 1).sum();
    let beacons_in_line = beacons.iter().filter(|b| b.1 == POSITION_Y).count();
    let result = excluded_positions_count as usize - beacons_in_line;

    println!("Beacon can't be in {} positions", result);
}
