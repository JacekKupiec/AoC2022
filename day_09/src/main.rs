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

fn get_vector_from_coomand(command: &str) -> (isize, isize) {
    match command {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("Unknown argument: {}", command)
    }
}

fn main() {
    let file = File::open("small_input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut tail_positions: HashSet<Position> = HashSet::new();
    let mut head: Position = Default::default();
    let mut tail: Position = Default::default();

    tail_positions.insert(tail);

    for line in reader.lines().map(|l| l.unwrap()) {
        let parts: Vec<&str> = line.trim_end().split(" ").collect();
        let (x_command_move, y_command_move) = get_vector_from_coomand(parts[0]);
        let repeats_count: usize = parts[1].parse().unwrap();

        for _ in 0..repeats_count {
            head.vector_move(x_command_move, y_command_move);

            if tail.is_not_adjacent(&head) {
                // move tail properly
                let (x_distance, y_distance) = head - tail;
                tail.vector_move(x_distance, y_distance);

                tail_positions.insert(tail);
            }
        }
    }

    println!("Answer: {}", tail_positions.len());
}
