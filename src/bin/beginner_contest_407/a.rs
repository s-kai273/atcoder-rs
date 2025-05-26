use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let a = parts[0];
    let b = parts[1];
    let quotient = a / b;
    let rounded = if quotient.fract() > 0.5 {
        quotient.trunc() + 1.0
    } else {
        quotient.trunc()
    };
    println!("{}", rounded);
}
