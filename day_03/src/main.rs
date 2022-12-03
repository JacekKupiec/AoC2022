use std::{io::{BufReader, BufRead}, fs::File, collections::HashSet};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);


    let elfs_rucksacks: Vec<HashSet<u8>> = reader.lines()
        .map(|line| {
            let line_unwraped = line.unwrap();
            let line_ascii = line_unwraped.trim_end().as_bytes();

            line_ascii.iter().map(|x| *x).collect()
        })
        .collect();

    let result: i32 = elfs_rucksacks.chunks_exact(3)
        .map(|chunk| {
            let elf1 = &chunk[0];
            let elf2 = &chunk[1];
            let elf3 = &chunk[2];
            let common12: HashSet<u8> = elf1.intersection(elf2).map(|e| *e).collect();
            
            let badge = *common12.intersection(elf3).last().unwrap();

            if badge.is_ascii_lowercase() {
                (badge - b'a' + 1) as i32
            } else {
                (badge - b'A' + 27) as i32
            }
        })
        .sum();

        println!("{}", result);
}
