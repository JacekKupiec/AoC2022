use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Copy, Clone)]
enum WindDirection {
    Left, Right
}

#[derive(Debug, Copy, Clone, Hash)]
struct Point(i32, i32);

const CHAMBER_WIDTH: i32 = 7;
const CHAMBER_LEFT_WALL: i32 = 0;
const CHAMBER_BOTTOM_WALL: i32 = 0;
const CHAMBER_RIGHT_WALL: i32 = CHAMBER_LEFT_WALL + CHAMBER_WIDTH - 1;

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point(x, y)
    }

    fn move_down(&mut self) {
        self.1 = max(self.1 - 1, CHAMBER_BOTTOM_WALL + 1);
    }

    fn move_left(&mut self) {
        self.x = max(self.x - 1, 0);
    }

    fn move_right(&mut self) {
        self.x = min(self.x + 1, CHAMBER_WIDTH - 1);
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
    fn new(rock_type: RockType, height: i32) -> Rock {
        const OFFSET_FROM_TOP: i32 = 4;

        match rock_type {
            RockType::Minus => Rock {
                rock_pieces: vec![
                    Point(2, height + OFFSET_FROM_TOP),
                    Point(3, height + OFFSET_FROM_TOP),
                    Point(4, height + OFFSET_FROM_TOP),
                    Point(5, height + OFFSET_FROM_TOP)
                ]
            },
            RockType::Plus => Rock {
                rock_pieces: vec![
                    Point(3, height + OFFSET_FROM_TOP),
                    Point(3, height + OFFSET_FROM_TOP + 1),
                    Point(3, height + OFFSET_FROM_TOP + 2),
                    Point(2, height + OFFSET_FROM_TOP + 1),
                    Point(4, height + OFFSET_FROM_TOP + 1)
                ]
            },
            RockType::L => Rock {
                rock_pieces: vec![
                    Point(2, height + OFFSET_FROM_TOP),
                    Point(3, height + OFFSET_FROM_TOP),
                    Point(4, height + OFFSET_FROM_TOP),
                    Point(4, height + OFFSET_FROM_TOP + 1),
                    Point(4, height + OFFSET_FROM_TOP + 1),
                ]
            },
            RockType::Pipe => Rock {
                rock_pieces: vec![
                    Point(2, height + OFFSET_FROM_TOP),
                    Point(2, height + OFFSET_FROM_TOP + 1),
                    Point(2, height + OFFSET_FROM_TOP + 2),
                    Point(2, height + OFFSET_FROM_TOP + 3)
                ]
            },
            RockType::Square => Rock {
                rock_pieces: vec![
                    Point(2, height + OFFSET_FROM_TOP),
                    Point(2, height + OFFSET_FROM_TOP + 1),
                    Point(3, height + OFFSET_FROM_TOP),
                    Point(3, height + OFFSET_FROM_TOP + 1)
                ]
            }
        }
    }

    fn can_move_down(&self, rock_heap: &RockHeap) -> bool {
        self.rock_pieces.iter().all(|Point(_, y)| *y > 1)
        || !self.rock_pieces.iter().any(|Point(x, y)| rock_heap.heap.contains_key(Point(*x, y - 1)))
    }

    fn move_down(&mut self, rock_heap: &RockHeap) {
        self.rock_pieces.iter_mut().for_each(|p| p.move_down())
    }

    fn can_move_horizontally(&self, rock_heap: &RockHeap) {
        todo!()
    }

    fn move_horizontally(&mut self, wind_direction: WindDirection, rock_heap: &RockHeap) {
        todo!()
    }
}

struct RockHeap {
    heap: HashSet<Point>,
    max_height: i32
}

impl RockHeap {
    fn new() -> RockHeap {
        RockHeap {
            heap: Default::default(),
            max_height: 0
        }
    }

    fn add_rock(&self, p0: Rock) {
        todo!()
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
    let mut wind = buffer.trim_end().bytes();
    let mut rock_heap = RockHeap::new();

    for rock_type in rocks_falling.iter().cycle().take(2022) {
        let mut rock = Rock::new(*rock_type, rock_heap.max_height);

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

            rock.move_horizontally(wind_direction, &rock_heap);

            if !rock.move_down(&rock_heap) {
                rock_heap.add_rock(rock);
                break;
            }
        }
    }
}
