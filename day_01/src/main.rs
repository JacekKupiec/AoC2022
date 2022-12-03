use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut buffer = String::new();
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut calories_list_by_elf : Vec<Vec<i32>> = vec![Vec::new()];
    let mut current_elf = 0;

    while let Ok(bytes_read) = reader.read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        if buffer.starts_with("\r\n") {
            current_elf += 1;
            calories_list_by_elf.push(Vec::new());
        } else {
            let calories = buffer.trim_end().parse().unwrap();
            calories_list_by_elf[current_elf].push(calories);
        }

        buffer.clear();
    }

    let mut calories_for_each_elf: Vec<i32> = calories_list_by_elf.iter()
        .map(|calories| calories.iter().sum::<i32>()*-1)
        .collect();

    calories_for_each_elf.sort_unstable();
    let result = calories_for_each_elf.iter().take(3).sum::<i32>()*-1;

    println!("{}", result);
}
