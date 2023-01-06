use std::fs::File; 
use std::io::{BufReader, BufRead};
use std::collections::VecDeque;

use scanf::sscanf;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add(i32),
    Multiply(i32),
    Squere
}

struct Monkey {
    items: VecDeque<i32>,
    operation: Operation,
    divisor: i32,
    idx_if_divisible: usize,
    idx_if_not_divisible: usize,
    inspection_count: usize
}

fn parse_starting_items(line: &str) -> VecDeque<i32> {
    let (_, starting_items) = line
        .trim()
        .split_once(": ")
        .unwrap();

    starting_items
        .trim()
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect()
}

fn parse_operation(line: &str) -> Operation {
    let mut operator: char = '\0';
    let mut operand = String::new();
    
    let _ = sscanf!(line.trim(), "Operation: new = old {} {}", operator, operand);

    if operator == '+' {
        Operation::Add(operand.parse().unwrap())
    } else if operand == "old" {
        Operation::Squere
    } else {
        Operation::Multiply(operand.parse().unwrap())
    }
}

fn parse_divisor(line: &str) -> i32 {
    let mut divisor = 0;

    let _ = sscanf!(line.trim(), "Test: divisible by {}", divisor);

    divisor
}

fn parse_target_monkeys(lines: &str) -> (usize, usize) {
    let numbers: Vec<usize> = lines
        .lines()
        .map(|line| { 
            let mut text_to_skip = String::new();
            let mut monkey_number = 0;

            let _ = sscanf!(line.trim(), "If {}: throw to monkey {}", text_to_skip, monkey_number);

            monkey_number
        })
        .take(2)
        .collect();

    (numbers[0], numbers[1])
}

fn main() {
    let file = File::open("small_.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut monkeys: Vec<Monkey> = Vec::new();
    
    // read all maonkeys' data
    loop {
        // skip monkey's number
        let bytes_read = reader.read_line(&mut buffer).unwrap();
        buffer.clear();

        if bytes_read == 0 {
            break;
        }

        let _ = reader.read_line(&mut buffer).unwrap();
        let monkeys_items = parse_starting_items(&buffer);
        buffer.clear();

        let _ = reader.read_line(&mut buffer);
        let operation = parse_operation(&buffer);        
        buffer.clear();

        let _ = reader.read_line(&mut buffer);
        let divisor = parse_divisor(&buffer);
        buffer.clear();

        let _ = reader.read_line(&mut buffer);
        let _ = reader.read_line(&mut buffer);
        let (divisible, not_divisible) = parse_target_monkeys(&buffer);

        let monkey = Monkey {
            items: monkeys_items,
            operation,
            divisor,
            idx_if_divisible: divisible,
            idx_if_not_divisible: not_divisible,
            inspection_count: 0
        };
        monkeys.push(monkey);

        // skip blank line
        let _ = reader.read_line(&mut buffer);
    }

    for _ in 1..=10_000 {
        for monkey_idx in 0..monkeys.len() {
            monkeys[monkey_idx].inspection_count += monkeys[monkey_idx].items.len();

            while let Some(item_worry_level) = monkeys[monkey_idx].items.pop_front() {
                let op = monkeys[monkey_idx].operation;

                let re_evaluated_item = match op {
                    Operation::Add(term) => item_worry_level + term,
                    Operation::Multiply(factor) => item_worry_level * factor,
                    Operation::Squere => item_worry_level.pow(2)
                } ??;

                if re_evaluated_item % monkeys[monkey_idx].divisor == 0 {
                    let idx_if_divisible = monkeys[monkey_idx].idx_if_divisible;
                    monkeys[idx_if_divisible].items.push_back(re_evaluated_item);
                } else {
                    let idx_if_not_divisible = monkeys[monkey_idx].idx_if_not_divisible;
                    monkeys[idx_if_not_divisible].items.push_back(re_evaluated_item);
                }
            }
        }
    }

    monkeys.sort_by(|a, b| 
        a.inspection_count.cmp(&b.inspection_count).reverse());

    println!("{}", monkeys[0].inspection_count * monkeys[1].inspection_count);

}
