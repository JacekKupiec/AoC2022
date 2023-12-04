use std::fs::File;
use std::io::Read;

fn main() {
    let path = "input.txt";
    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();

    if let Err(message) = file.read_to_string(&mut buffer) {
        println!("Error message: {}", message)
    }
}
