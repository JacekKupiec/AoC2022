use chrono::DateTime;

#[derive(Debug)]
struct MyDate {
    day: u8,
    month: u8,
    year: u16
}

impl MyDate {
    fn new(day: u8, month: u8, year: u16) -> Self {
        MyDate {
            day, month, year
        }
    }
}

fn main() {
    let mut t = vec![
        MyDate::new(31, 1, 2025),
        MyDate::new(29, 1, 2021),
        MyDate::new(30, 2, 2021),
        MyDate::new(27, 2, 2021),
        MyDate::new(24, 3, 2021),
        MyDate::new(23, 3, 2021)
    ];

    println!("{:?}", t);
    t.sort_by(|a, b| a.day.cmp(&b.day));
    println!("{:?}", t);
    t.sort_by(|a, b| a.month.cmp(&b.month));
    println!("{:?}", t);
    t.sort_by(|a, b| a.year.cmp(&b.year));
    println!("{:?}", t);
}
