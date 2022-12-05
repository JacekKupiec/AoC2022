use std::fs::{File};
use std::io::{BufRead, BufReader};
use scanf::sscanf;

const STACKS_NUM: usize = 9;
const HEIGHT: usize = 8;

fn main() {
    let mut buffer = String::new();
    let mut stacks: Vec<Vec<u8>> = vec![vec![]; STACKS_NUM];
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    for _ in 0..HEIGHT {
        let _ = reader.read_line(&mut buffer);
        let bytes: Vec<u8> = buffer.bytes().collect();

        for j in 0..STACKS_NUM {
            if bytes.len() <= 1 + j*4 {
                break;
            }

            let element = bytes[1 + j*4];

            if element.is_ascii_alphabetic() {
                stacks[j].push(element);
            }
        }

        buffer.clear();
    }

    let _ = reader.read_line(&mut buffer);
    let _ = reader.read_line(&mut buffer);
    buffer.clear();

    for stack in &mut stacks {
        stack.reverse();
    }

    while let Ok(bytes_read) = reader.read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        let mut which: usize = 123;
        let mut from: usize = 123;
        let mut to: usize = 123;

        let _ = sscanf!(buffer.trim_end(), "move {} from {} to {}", which, from, to);
        let mut temp_buffer : Vec<u8> = Vec::with_capacity(which);

        for _ in 0..which {
            let x = stacks[from - 1].pop().unwrap();
            temp_buffer.push(x);
        }

        //temp_buffer.reverse();

        for _ in 0..which {
            let x = temp_buffer.pop().unwrap();
            stacks[to - 1].push(x);
        }

        buffer.clear();
    }

    for stack in &stacks {
        print!("{}", *stack.last().unwrap() as char);
    }
}
