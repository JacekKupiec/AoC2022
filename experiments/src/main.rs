use std::collections::HashSet;

fn main() {
    let t = [1, 2, 3, 4, 5];

    for i in t.iter().skip(2).cycle().take(10) {
        println!("{}", i);
    }
}
