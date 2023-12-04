use std::collections::HashSet;

fn main() {
    let s = "str";
    let s2 = String::from("strstr");
    let set = HashSet::from([s]);

    println!("{} {}", &s2[0..=2], set.contains(&s2[0..=2]));

    let x = -17;
    println!("{:#03b}\n{:#034b}", x, !x);
}
