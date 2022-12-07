use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const MARKER_LENGTH: usize = 14;

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    if let Err(message) = reader.read_line(&mut buffer) {
        eprintln!("The error occured. Can't process: {}", message);
        return;
    }

    let position = buffer.trim_end().as_bytes().windows(MARKER_LENGTH).position(|window| {
        let mut set = HashSet::with_capacity(MARKER_LENGTH);
        let mut range = 0..MARKER_LENGTH;

        return range.all(|idx| set.insert(window[idx]));
    })
    .unwrap();

    println!("{}", position + MARKER_LENGTH);
}
