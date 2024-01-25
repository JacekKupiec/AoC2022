use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::iter::{Cycle, Enumerate};
use std::slice::Iter;
use std::str::Bytes;

#[derive(Debug, Copy, Clone)]
enum WindDirection {
    Left, Right
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64
}

const CHAMBER_LEFT_WALL: i64 = 0;
const CHAMBER_BOTTOM_WALL: i64 = 0;
const CHAMBER_RIGHT_WALL: i64 = 8;

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    fn move_down(&mut self) {
        self.y = self.y - 1;
    }

    fn move_left(&mut self) {
        self.x = self.x - 1;
    }

    fn move_right(&mut self) {
        self.x = self.x + 1;
    }
}

struct Rock {
    rock_pieces: Vec<Point>
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RockType {
    Minus,
    Plus,
    L,
    Pipe,
    Square
}

impl Rock {
    fn new(rock_type: RockType, height: i64) -> Rock {
        const OFFSET_FROM_TOP: i64 = 4;

        match rock_type {
            RockType::Minus => Rock {
                rock_pieces: vec![
                    Point::new(3, height + OFFSET_FROM_TOP),
                    Point::new(4, height + OFFSET_FROM_TOP),
                    Point::new(5, height + OFFSET_FROM_TOP),
                    Point::new(6, height + OFFSET_FROM_TOP)
                ]
            },
            RockType::Plus => Rock {
                rock_pieces: vec![
                    Point::new(4, height + OFFSET_FROM_TOP),
                    Point::new(4, height + OFFSET_FROM_TOP + 1),
                    Point::new(4, height + OFFSET_FROM_TOP + 2),
                    Point::new(3, height + OFFSET_FROM_TOP + 1),
                    Point::new(5, height + OFFSET_FROM_TOP + 1)
                ]
            },
            RockType::L => Rock {
                rock_pieces: vec![
                    Point::new(3, height + OFFSET_FROM_TOP),
                    Point::new(4, height + OFFSET_FROM_TOP),
                    Point::new(5, height + OFFSET_FROM_TOP),
                    Point::new(5, height + OFFSET_FROM_TOP + 1),
                    Point::new(5, height + OFFSET_FROM_TOP + 2),
                ]
            },
            RockType::Pipe => Rock {
                rock_pieces: vec![
                    Point::new(3, height + OFFSET_FROM_TOP),
                    Point::new(3, height + OFFSET_FROM_TOP + 1),
                    Point::new(3, height + OFFSET_FROM_TOP + 2),
                    Point::new(3, height + OFFSET_FROM_TOP + 3)
                ]
            },
            RockType::Square => Rock {
                rock_pieces: vec![
                    Point::new(3, height + OFFSET_FROM_TOP),
                    Point::new(3, height + OFFSET_FROM_TOP + 1),
                    Point::new(4, height + OFFSET_FROM_TOP),
                    Point::new(4, height + OFFSET_FROM_TOP + 1)
                ]
            }
        }
    }

    fn move_down(&mut self) {
        self.rock_pieces.iter_mut().for_each(|p| p.move_down())
    }

    fn move_horizontally(&mut self, wind_direction: WindDirection) {
        match wind_direction {
            WindDirection::Left =>
                self.rock_pieces.iter_mut().for_each(|point| point.move_left()),
            WindDirection::Right =>
                self.rock_pieces.iter_mut().for_each(|point| point.move_right())
        }
    }
}

struct RockHeap {
    heap: HashSet<Point>,
    max_height: i64
}

impl RockHeap {
    fn new() -> RockHeap {
        RockHeap {
            heap: Default::default(),
            max_height: 0
        }
    }

    fn can_move_horizontally(&self, rock: &Rock, wind_direction: WindDirection) -> bool {
        match wind_direction {
            WindDirection::Left =>
                rock.rock_pieces.iter().all(|point| point.x - 1 > CHAMBER_LEFT_WALL)
                    && !rock.rock_pieces.iter().any(|point| {
                    let moved_point = Point {
                        x: point.x - 1,
                        y: point.y
                    };

                    self.heap.contains(&moved_point)
                }),
            WindDirection::Right =>
                rock.rock_pieces.iter().all(|point| point.x + 1 < CHAMBER_RIGHT_WALL)
                    && !rock.rock_pieces.iter().any(|point| {
                    let moved_point = Point {
                        x: point.x + 1,
                        y: point.y
                    };

                    self.heap.contains(&moved_point)
                }),
        }
    }

    fn can_move_down(&self, rock: &Rock) -> bool {
        rock.rock_pieces.iter().all(|point| point.y - 1 > CHAMBER_BOTTOM_WALL) // is on the ground
        && !rock.rock_pieces.iter().any(|point| {
            let moved_point = Point {
                x: point.x,
                y: point.y - 1
            };
            self.heap.contains(&moved_point)
        })
    }

    fn add_rock(&mut self, rock: Rock) {
        for point in rock.rock_pieces {
            if self.max_height < point.y {
                self.max_height = point.y;
            }

            self.heap.insert(point);
        }
    }
}

fn run_to_zero(
    rock_heap: &mut RockHeap,
    wind: &mut Enumerate<Cycle<Bytes>>,
    rocks_falling: &mut Cycle<Iter<RockType>>,
    wind_period: usize,
    max_elements_count : usize,
    stop_element: usize)
-> Vec<i64>
{
    let mut heap_height_with_wind = Vec::new();

    'rock_loop: for rock_type in rocks_falling.take(max_elements_count) {
        let mut rock = Rock::new(*rock_type, rock_heap.max_height);

        loop {
            let (wind_idx, wind_direction) = if let Some((idx, c)) = wind.next() {
                let direction = match c {
                    b'<' => WindDirection::Left,
                    b'>' => WindDirection::Right,
                    _ => panic!("Unsupported character! Not < or >")
                };

                (idx, direction)
            } else {
                panic!("End of wind");
            };

            if rock_heap.can_move_horizontally(&rock, wind_direction) {
                rock.move_horizontally(wind_direction);
            }

            if rock_heap.can_move_down(&rock) {
                rock.move_down();
            } else {
                rock_heap.add_rock(rock);

                heap_height_with_wind.push(rock_heap.max_height);

                if (wind_idx % wind_period == stop_element) {
                    break 'rock_loop;
                } else {
                    break;
                }
            }
        }
    }

    return heap_height_with_wind;
}

fn main() {
    let path = "input.txt";
    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();

    if let Err(message) = file.read_to_string(&mut buffer) {
        println!("Error message: {}", message)
    }

    let rock_types = [RockType::Minus, RockType::Plus, RockType::L, RockType::Pipe, RockType::Square];
    let mut rocks_falling = rock_types.iter().cycle();
    let wind_period = buffer.len();
    let mut wind = buffer.bytes().cycle().enumerate();
    let mut rock_heap = RockHeap::new();
    const STEPS_NUMBER: i64 = 1000000000000;

    let mut steps_unique = run_to_zero(
        &mut rock_heap,
        &mut wind,
        &mut rocks_falling,
        wind_period,
        STEPS_NUMBER as usize,
        6);
    let mut steps_repeated = run_to_zero(
        &mut rock_heap,
        &mut wind,
        &mut rocks_falling,
        wind_period,
        STEPS_NUMBER as usize,
        6);

    // let mut file_output = File::create("output.csv").unwrap();
    // let _ = file_output.write("Height,Wind\n".as_bytes());
    //
    // for (h, i) in steps_unique {
    //     let content = format!("{},{}\n", h, i);
    //     let _ = file_output.write(content.as_bytes());
    // }

    let base_level = steps_unique.last().unwrap();
    let delta = steps_repeated.last().unwrap() - base_level;
    let steps_left = STEPS_NUMBER - steps_unique.len() as i64;
    let cycle_length = steps_repeated.len() as i64;
    let division = steps_left / cycle_length;
    let reminder = steps_left % cycle_length;

    let reminder_height = if reminder > 0 { steps_repeated[(reminder - 1) as usize] - base_level } else { 0 };
    let height = steps_unique.last().unwrap() + division*delta + reminder_height;

    println!("Height: {}", height);
}
