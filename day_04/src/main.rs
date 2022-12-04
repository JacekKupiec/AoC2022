use std::{io::{BufReader, BufRead}, fs::File};

use scanf::sscanf;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let result = reader.lines().filter(|line| {
        let line_unwrap = line.as_ref().unwrap();
        let mut section_assignment_left1 = 0;
        let mut section_assignment_right1 = 0;
        let mut section_assignment_left2 = 0;
        let mut section_assignment_right2 = 0;

        let _ = sscanf!(line_unwrap.trim_end(), "{i32}-{i32},{i32}-{i32}", 
            section_assignment_left1, 
            section_assignment_right1, 
            section_assignment_left2, 
            section_assignment_right2);

        let sections_not_overlap = (section_assignment_left1 < section_assignment_left2 && section_assignment_right1 < section_assignment_left2)
            || (section_assignment_left2 < section_assignment_left1 && section_assignment_right2 < section_assignment_left1);

        return !sections_not_overlap;
    })
    .count();

    println!("{}", result);
}
