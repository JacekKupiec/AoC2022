use std::collections::HashSet;
use std::io::{BufReader, BufRead};
use std::fs::{File, read};
use std::ops::Deref; 

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop,
    Add(i32)
}

fn solution_part1(reader: BufReader<File>, desired_cycles: &HashSet<i32>) -> i32 {
    reader.lines()
        //.chain(once(Ok(String::from("noop"))))
        .map(|l| {
            let line = l.unwrap();
            let instruction_parts: Vec<_> = line.trim_end().split(' ').collect();

            match instruction_parts[0] {
                "noop" => Instruction::Noop,
                "addx" => Instruction::Add(instruction_parts[1].parse().unwrap()),
                _ => panic!("Cannot recognise {:?}", instruction_parts)
            }
        })
        .scan(1, |register_state, instruction| {
            match instruction {
                Instruction::Noop => {
                    Some(Vec::from([*register_state]))
                },
                Instruction::Add(argument) => {
                    let previous_state = *register_state;

                    *register_state += argument;
                    Some(Vec::from([previous_state, previous_state]))
                }
            }
        })
        .flatten()
        .take(220)
        .zip(1..)
        .filter_map(|(register_state, cycle)| {
            if desired_cycles.contains(&cycle) {
                Some(register_state * cycle)
            } else {
                None
            }
        })
        .sum()
}

fn solution_part2(reader: BufReader<File>) {
    reader.lines()
        .map(|l| {
            let line = l.unwrap();
            let instruction_parts: Vec<_> = line.trim_end().split(' ').collect();

            match instruction_parts[0] {
                "noop" => Instruction::Noop,
                "addx" => Instruction::Add(instruction_parts[1].parse().unwrap()),
                _ => panic!("Cannot recognise {:?}", instruction_parts)
            }
        })
        .scan(1, |register_state, instruction| {
            match instruction {
                Instruction::Noop => {
                    Some(Vec::from([*register_state]))
                },
                Instruction::Add(argument) => {
                    let previous_state = *register_state;

                    *register_state += argument;
                    Some(Vec::from([previous_state, previous_state]))
                }
            }
        })
        .flatten()
        .take(240)
        .enumerate()
        .for_each(|(crt_cycle, register_state)| {
            if crt_cycle % 40 == 0{
                print!("\n");
            }

            let crt_cycle = (crt_cycle % 40) as i32;

            if register_state >= crt_cycle - 1 && register_state <= crt_cycle + 1 {
                print!("#");
            } else {
                print!(" ");
            }
        })
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let desired_cycles = (20..).step_by(40).take_while(|&c| c <= 220);
    let desired_cycles_lookup: HashSet<i32> = HashSet::from_iter(desired_cycles);

    // let result: i32 = solution_part1(&reader, &desired_cycles_lookup);
    // println!("Answer: {:?}", result);

    solution_part2(reader);
}
