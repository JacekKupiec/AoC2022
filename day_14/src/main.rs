use std::cmp::{max, min};
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::repeat;
use std::str::FromStr;

/* Here each point is a separate element in a set. This approach disenables add a infinite horizontal line.
 To make it easy I should have stored each line as a struct and for each unit of sand after each move
  I should have been iterating over all lines and check if I the sand touch the rock. This definitely
  slows down the execution time but allows to add an infinite line easily. */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x : i32,
    y : i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {
            x, y
        }
    }

    fn next_down(&self) -> Point {
        Point {
            x: self.x, y: self.y + 1
        }
    }

    fn next_left(&self) -> Point {
        Point {
            x: self.x - 1, y: self.y + 1
        }
    }

    fn next_right(&self) -> Point {
        Point {
            x: self.x + 1, y: self.y + 1
        }
    }

    fn fall_down(&mut self) {
        self.y += 1;
    }

    fn fall_left(&mut self) {
        self.x -= 1;
        self.y += 1;
    }

    fn fall_rigth(&mut self) {
        self.x += 1;
        self.y += 1;
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
    let path = "D:\\source\\Rust\\AoC 2022\\day_14\\src\\input.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut cave_rock_structure: HashSet<Point> = reader.lines()
        .flat_map(|line| {
            // These are points in a single line that are separated by " -> "
            let crucial_points: Vec<Point> = line.unwrap()
                .trim_end()
                .split(" -> ")
                .map(|s|  s.parse().unwrap())
                .collect();

            // Generate all points in the line 498,4 -> 498,6 including the edges
            // This will generate duplicates but the will be deleted by HashSet (set has no duplicates)
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

    let mut sand_in_rest = 0;
    let cave_bed = cave_rock_structure.iter().map(|p| p.y).max().unwrap() + 1;
    let pouring_point = Point::new(500, 0);

    'sand_falling_process: loop {
        let mut falling_sand = pouring_point;

        'unit_of_sand_falling: loop {
            if falling_sand.y == cave_bed {
                sand_in_rest += 1;
                cave_rock_structure.insert(falling_sand);
                break 'unit_of_sand_falling;
            }

            let down_move = falling_sand.next_down();

            if !cave_rock_structure.contains(&down_move) {
                falling_sand.fall_down();
                continue 'unit_of_sand_falling;
            }

            let left_move = falling_sand.next_left();

            if !cave_rock_structure.contains(&left_move) {
                falling_sand.fall_left();
                continue 'unit_of_sand_falling;
            }

            let right_move = falling_sand.next_right();

            if !cave_rock_structure.contains(&right_move) {
                falling_sand.fall_rigth();
                continue 'unit_of_sand_falling;
            }

            sand_in_rest += 1;
            cave_rock_structure.insert(falling_sand);

            if falling_sand == pouring_point {
                break 'sand_falling_process;
            } else {
                break 'unit_of_sand_falling;
            }
        }
    }

    println!("{}", sand_in_rest);
}
