use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let _: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let s: String = lines.next().unwrap().unwrap();
    let t: String = lines.next().unwrap().unwrap();
    let answer: usize = s.chars().zip(t.chars()).filter(|(a, b)| a != b).count();
    println!("{}", answer);
}
