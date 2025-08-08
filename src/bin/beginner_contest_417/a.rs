use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let a: usize = iter.next().unwrap().parse().unwrap();
    let b: usize = iter.next().unwrap().parse().unwrap();
    let s: String = lines.next().unwrap().unwrap().trim().to_string();
    let mut chars: Vec<char> = s.chars().collect();
    chars.drain(..a);
    chars.drain(n - a - b..);
    println!("{}", chars.iter().collect::<String>());
}
