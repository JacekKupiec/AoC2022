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
    let x = (4.0f64 - 15.0f64) / 0.0;
    println!("{} {}", x + 4.0, 3.0 / x);

    let w = vec![1.0, 12.0, 4.5];
    let dw = derivate(&w);
    let ddw = derivate(&dw);
    let ddw_value = evaluate_polynomial(&ddw, 7f64);

    println!("{:?} {:?} {:?}", w, dw, ddw);
    println!("{:?}", ddw_value);
}
