use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).unwrap();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    
    let _ = reader.lines()
        .filter_map(|line_result| {
            let line = line_result.unwrap().to_string();

            if !line.is_empty() {
                Some(line.to_string())
            } else {
                None
            }
        })
        .array_chunks()
        .filter_map(|[left_line, right_line]| {
            None
        });
}
