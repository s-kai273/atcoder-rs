use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let p = lines.next().unwrap().unwrap();
    let l: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let answer = if p.len() >= l { "Yes" } else { "No" };
    println!("{}", answer);
}
