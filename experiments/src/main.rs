use std::f64::consts::PI;
use chrono::DateTime;

fn derivate(polynomial: &Vec<f64>) -> Vec<f64> {
    let deg = polynomial.len();

    (1..deg).rev()
        .enumerate()
        .map(|(idx, n)| n as f64 * polynomial[idx])
        .collect()
}

fn evaluate_polynomial(coefficients: &Vec<f64>, r: f64) -> f64 {
    let mut result = coefficients[0];
    let r = r + 1.0;

    for coefficient in coefficients.iter().skip(1) {
        result = result*r + coefficient;
    }

    return result;
}

fn main() {
    let i = -0;
    let x = -i..=i;

    println!("{}", x.count());
}
