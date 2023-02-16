#![feature(iter_array_chunks)]

use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::SignalPacket::*;

#[derive(Debug, Eq)]
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
    let mut reval: Vec<SignalPacket> = Vec::new();

    if let Some(text_prefix_stripped) = input_text.strip_prefix('[') {
        if let Some(stripped_text) = text_prefix_stripped.strip_suffix(']') {
            for slice in stripped_text.split(',') {
                if slice.starts_with('[') {
                    let result = parse_list(slice);

                    reval.push(List(result));
                } else if let Ok(number) = slice.parse() {
                    reval.push(Number(number));
                }
            }
        }
    }

    reval
}

fn main() {
    let args: Vec<String> = env::args().collect();
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

            if left_list.lt(&right_list) {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum();

    println!("{:?}", results);
}
