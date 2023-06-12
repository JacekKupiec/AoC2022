use std::cmp::{max, min};
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::repeat;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x : i32,
    y : i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coordinates: Vec<_> = s.split(',').collect();

        return Ok(Point {
            x: coordinates[0].parse().unwrap(),
            y: coordinates[1].parse().unwrap()
        });
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //let path = "D:\\source\\Rust\\AoC 2022\\day_13\\small_input.txt";
    let path = args.get(1).unwrap();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let parsed_points: HashSet<Point> = reader.lines()
        .flat_map(|line| {
            let crucial_points: Vec<Point> = line.unwrap()
                .trim_end()
                .split(" -> ")
                .map(|s|  s.parse::<Point>().unwrap())
                .collect();

            let all_points_in_the_line : Vec<Point> = crucial_points.windows(2)
                .flat_map(|wnd| {
                    let begin = wnd[0];
                    let end = wnd[1];
                    let make_point = |(x, y)| Point::new(x, y);

                    let iterator : Box<dyn Iterator<Item=Point>> = if begin.x == end.x {
                        let range_x = repeat(begin.x);
                        let range_y = min(begin.y, end.y)..=max(begin.y, end.y);

                        Box::new(range_x.zip(range_y).map(make_point))
                    } else {
                        let range_x = min(begin.x, end.x)..=max(begin.x, end.x);
                        let range_y = repeat(begin.y);

                        Box::new(range_x.zip(range_y).map(make_point))
                    };

                    iterator
                })
                .collect();

            return all_points_in_the_line;
        })
        .collect();

    
}
