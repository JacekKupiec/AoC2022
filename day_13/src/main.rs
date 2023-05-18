#![feature(iter_array_chunks)]

// this works with array chunks feature, as far as I know it requres nightly toolchain

use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::SignalPacket::*;

#[derive(Debug, Eq, Clone)]
enum SignalPacket{
    Number(i32),
    List(Vec<SignalPacket>)
}

impl PartialEq for SignalPacket {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number(x), Number(y)) => x.eq(y),
            (List(v1), Number(n)) => {
                let v2 = vec![Number(*n)];
                v1.eq(&v2)
            },
            (Number(n), List(v2)) => {
                let v1 = vec![Number(*n)];
                v1.eq(v2)
            },
            (List(v1), List(v2)) => v1.eq(v2)
        }
    }
}

impl PartialOrd for SignalPacket {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SignalPacket {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Number(x), Number(y)) => x.cmp(y),
            (List(v1), Number(n)) => {
                let v2 = vec![Number(*n)];
                v1.cmp(&v2)
            },
            (Number(n), List(v2)) => {
                let v1 = vec![Number(*n)];
                v1.cmp(v2)
            },
            (List(v1), List(v2)) => v1.cmp(v2)
        }
    }
}

fn parse_list(input_text: &str) -> Vec<SignalPacket> {
    let mut nested_arrays : Vec<Vec<SignalPacket>> = Vec::new();
    let mut buffer = String::new();
    let mut characters = input_text.chars();

    while let Some(c) = characters.next() {
        match c {
            '[' => nested_arrays.push(Vec::new()),
            '0'..='9' => buffer.push(c),
            ',' => {
                let most_nested = nested_arrays.last_mut().unwrap();

                if !buffer.is_empty() {
                    most_nested.push(Number(buffer.parse().unwrap()));
                    buffer.clear();
                }
            },
            ']' => {
                let mut mostly_nested_set = nested_arrays.pop().unwrap();

                if !buffer.is_empty() {
                    mostly_nested_set.push(Number(buffer.parse().unwrap()));
                    buffer.clear();
                }

                if let Some(parent_set) = nested_arrays.last_mut() {
                    parent_set.push(List(mostly_nested_set));
                } else {
                    return mostly_nested_set;
                }
                
            },
            _ => panic!("Unparsable character {}", c)
        }
    }

    panic!("Too many opening [, no closing ]");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //let path = "D:\\source\\Rust\\AoC 2022\\day_13\\small_input.txt";
    let path = args.get(1).unwrap();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    
    let results: usize = reader.lines()
        .filter_map(|line_result| {
            let line = line_result.unwrap().trim_end().to_string();

            if !line.is_empty() {
                Some(line)
            } else {
                None
            }
        })
        .array_chunks()
        .enumerate()
        .filter_map(|(idx, [left_line, right_line])| {
            let left_list = parse_list(&left_line);
            let right_list = parse_list(&right_line);

            // println!("{:?} <- {}\n{:?} <- {}\n{:?}\n\n", 
            //     left_list, left_line, 
            //     right_list, right_line, 
            //     left_list.cmp(&right_list));

            if left_list.lt(&right_list) {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum();

    println!("{:?}", results);
}
