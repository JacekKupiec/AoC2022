use std::iter::repeat;

fn main() {
    let x: Vec<(usize, usize)> = (1..5).zip(repeat(9)).collect();

    println!("{:?}", x);
}
