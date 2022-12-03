use std::{fs::File, io::{BufReader, BufRead}};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use crate::HandShape::*;
use crate::GameResult::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
enum HandShape {
    Rock,
    Paper,
    Scissors
}

impl TryFrom<u8> for HandShape {
    type Error = &'static str;

    fn try_from(hand_shape_code: u8) -> Result<Self, Self::Error> {
        match hand_shape_code {
            b'A' => Ok(Rock),
            b'B' => Ok(Paper),
            b'C' => Ok(Scissors),
            _ => Err("Value out of range")
        }
    }
}

impl Ord for HandShape {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Equal;
        }

        match *self {
            Rock => if *other == Scissors { Greater } else { Less },
            Paper => if *other == Rock { Greater } else { Less },
            Scissors => if *other == Paper { Greater } else { Less }
        }
    }
}

impl HandShape {
    fn get_score(self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn play_match(self, opponent: HandShape) -> i32 {
        match self.cmp(&opponent) {
            Less => self.get_score(),
            Equal => 3 + self.get_score(),
            Greater => 6 + self.get_score()
        }
    }

    fn pick_opponent_that(self, game_result: GameResult) -> HandShape {
        match game_result {
            Loss => {
                match self {
                    Rock => Scissors,
                    Paper => Rock,
                    Scissors => Paper,
                }
            }
            Draw => self,
            Win => {
                match self {
                    Rock => Paper,
                    Paper => Scissors,
                    Scissors => Rock
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum GameResult {
    Loss,
    Draw,
    Win
}

impl TryFrom<u8> for GameResult {
    type Error = &'static str;

    fn try_from(game_result_code: u8) -> Result<Self, Self::Error> {
        match game_result_code {
            b'X' => Ok(Loss),
            b'Y' => Ok(Draw),
            b'Z' => Ok(Win),
            _ => Err("Value out of range")
        }
    }
}

fn main() {
    let file = File::open("small_input.txt").unwrap();
    let reader = BufReader::new(file);

    let result : i32 = reader.lines().map(|l| {
        let line = l.unwrap().into_bytes();
        let left : HandShape = line[0].try_into().unwrap();
        let game_result: GameResult = line[2].try_into().unwrap();
        let my_shape = left.pick_opponent_that(game_result);

        my_shape.play_match(left)
    })
        .sum();

    println!("{}", result);
}
