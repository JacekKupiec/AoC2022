use std::cell::{Cell, RefCell};
use std::fs::File; 
use std::io::{BufReader, BufRead};
use std::collections::VecDeque;

use scanf::sscanf;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Squere
}

struct Monkey {
    items: RefCell<VecDeque<u64>>,
    operation: Operation,
    divisor: u64,
    idx_divisible: usize,
    idx_not_divisible: usize,
    inspection_count: Cell<u64>
}

fn parse_starting_items(line: &str) -> VecDeque<u64> {
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

fn parse_divisor(line: &str) -> u64 {
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
    let file = File::open("input.txt").unwrap();
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
            items: RefCell::new(monkeys_items),
            operation,
            divisor,
            idx_divisible: divisible,
            idx_not_divisible: not_divisible,
            inspection_count: Cell::new(0)
        };
        monkeys.push(monkey);

        // skip blank line
        let _ = reader.read_line(&mut buffer);
    }

    // all divisors are prime numbers so it's simpler to find the value
    let lowest_common_multiple: u64 = monkeys.iter().map(|m| m.divisor).product();

    for _ in 1..=10_000 {
        for monkey in &monkeys {
            //monkeys[monkey_idx].inspection_count += monkeys[monkey_idx].items.len();
            let items_count = monkey.items.borrow().len() as u64;
            let inspection_count = monkey.inspection_count.get();
            
            monkey.inspection_count.set(items_count + inspection_count);
            let mut monkey_items = monkey.items.borrow_mut();

            while let Some(item_worry_level) = monkey_items.pop_front() {
                let re_evaluated_item = match monkey.operation {
                    Operation::Add(term) => item_worry_level + term,
                    Operation::Multiply(factor) => item_worry_level * factor,
                    Operation::Squere => item_worry_level.pow(2)
                } % lowest_common_multiple;

                let target_monkey_idx = if re_evaluated_item % monkey.divisor == 0 {
                    monkey.idx_divisible
                } else {
                    monkey.idx_not_divisible
                };

                let mut target_monkey_items = monkeys[target_monkey_idx].items.borrow_mut();
                target_monkey_items.push_back(re_evaluated_item);
            }
        }
    }

    monkeys.sort_by(|a, b| 
        a.inspection_count.cmp(&b.inspection_count).reverse());

    println!("{}", monkeys[0].inspection_count.get() * monkeys[1].inspection_count.get());

}
