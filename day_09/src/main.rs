use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize
}

impl Position {
    fn is_not_adjacent(&self, other: &Position) -> bool {
        let x_distance = self.x - other.x;
        let y_distance = self.y - other.y;

        x_distance*x_distance + y_distance*y_distance > 2
    }

    fn vector_move(&mut self, x: isize, y: isize){
        self.x += x;
        self.y += y;
    }
}

// operator overloading
impl std::ops::Sub<Position> for Position {
    type Output = (isize, isize);

    fn sub(self, rhs: Position) -> Self::Output {
        let x = (self.x - rhs.x).clamp(-1, 1);
        let y = (self.y - rhs.y).clamp(-1, 1);

        (x, y)
    }
}

fn get_vector_from_comand(command: &str) -> (isize, isize) {
    match command {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("Unknown argument: {}", command)
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut tail_positions: HashSet<Position> = HashSet::new();
    let mut knots = [Position::default(); 10];

    tail_positions.insert(knots[9]);

    for line in reader.lines().map(|l| l.unwrap()) {
        let parts: Vec<&str> = line.trim_end().split(" ").collect();
        let (x_command_move, y_command_move) = get_vector_from_comand(parts[0]);
        let repeats_count: usize = parts[1].parse().unwrap();

        for _ in 0..repeats_count {
            knots[0].vector_move(x_command_move, y_command_move);

            for knot_idx in 1..knots.len() {
                let head_idx = knot_idx - 1;
                let tail_idx = knot_idx;

                if knots[tail_idx].is_not_adjacent(&knots[head_idx]) {
                    let (x_distance, y_distance) = knots[head_idx] - knots[tail_idx];
                    knots[tail_idx].vector_move(x_distance, y_distance);
                }
            }

            tail_positions.insert(*knots.last().unwrap());
        }
    }

    println!("Answer: {}", tail_positions.len());
}
