use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use scanf::sscanf;

fn manhattan_distance(x1 :i32, y1: i32, x2: i32, y2:i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn main() {
    let path = "D:\\source\\Rust\\AoC 2022\\day_15\\src\\input.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    const POSITION_Y: i32 = 2_000_000;
    let mut sensors: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut beacons: HashSet<(i32, i32)> = HashSet::new();
    let mut result: HashSet<(i32, i32)> = HashSet::new();

    for line in reader.lines().map(|l| l.unwrap() ) {
        let mut sensor_x = 0;
        let mut sensor_y = 0;
        let mut beacon_x = 0;
        let mut beacon_y = 0;

        let _ = sscanf!(line.as_str(), "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            sensor_x, sensor_y, beacon_x, beacon_y);

        let _ = sensors.insert((sensor_x, sensor_y), (beacon_x, beacon_y));
        let _ = beacons.insert((beacon_x, beacon_y));
    }

    for (sensor, beacon) in sensors.iter() {
        let distance = manhattan_distance(sensor.0, sensor.1, beacon.0, beacon.1);
        let distance_y = (sensor.1 - POSITION_Y).abs();
        let distance_x = distance - distance_y;

        let range = -distance_x..=distance_x;

        for point in range.map(|x| (sensor.0 + x, POSITION_Y)) {
            if !(sensors.contains_key(&point) || beacons.contains(&point)) {
                let _ = result.insert(point);
            }
        }
    }

    println!("Beacon can't be in {} positions", result.len());
}
