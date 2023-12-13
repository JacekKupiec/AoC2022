use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

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

#[derive(Debug, Copy, Clone)]
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

fn main() {
    let path = "input.txt";
    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();

    if let Err(message) = file.read_to_string(&mut buffer) {
        println!("Error message: {}", message)
    }

    let rocks_falling = [RockType::Minus, RockType::Plus, RockType::L, RockType::Pipe, RockType::Square];
    let mut wind = buffer.trim_end().bytes().cycle();
    let mut rock_heap = RockHeap::new();

    for rock_type in rocks_falling.iter().cycle().take(2022) {
        let mut rock = Rock::new(*rock_type, rock_heap.max_height);
        // let rock_size = rock.rock_pieces.len();

        loop {
            let wind_direction = if let Some(c) = wind.next() {
                match c {
                    b'<' => WindDirection::Left,
                    b'>' => WindDirection::Right,
                    _ => panic!("Unsupported character! Not < or >")
                }
            } else {
                break;
            };

            if rock_heap.can_move_horizontally(&rock, wind_direction) {
                rock.move_horizontally(wind_direction);
            }

            if rock_heap.can_move_down(&rock) {
                rock.move_down();
            } else {
                rock_heap.add_rock(rock);
                break;
            }
        }

        // println!("Heap size {}, max_height: {}, rock_length: {}, rock_type: {:?}",
        //          rock_heap.heap.len(),
        //          rock_heap.max_height,
        //          rock_size,
        //          rock_type);
    }

    println!("Tower height: {}", rock_heap.max_height);
}
