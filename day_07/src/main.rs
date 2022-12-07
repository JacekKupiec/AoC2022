use std::{io::{BufReader, BufRead}, fs::File};

fn main() {
    let mut buffer = String::new();
    let file = File::open("small_input.txt").unwrap();
    let mut reader = BufReader::new(file);

    while let Ok(bytes_read) = reader.read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let bytes = buffer.as_bytes();

        if bytes[0] == b'$' {

        } else {
            
        }

        buffer.clear();
    }
}
