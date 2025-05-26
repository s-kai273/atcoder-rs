use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let (x, y) = (parts[0], parts[1]);
    let mut count = 0;
    for i in 1..=6 {
        for j in 1..=6 {
            if i + j >= x || (i - j).abs() >= y {
                count += 1;
            }
        }
    }
    let answer = count as f64 / 36.0;
    println!("{:.10}", answer);
}
