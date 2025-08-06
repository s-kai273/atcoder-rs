use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let _: usize = iter.next().unwrap().parse().unwrap();
    let l: usize = iter.next().unwrap().parse().unwrap();
    let r: usize = iter.next().unwrap().parse().unwrap();
    let s: String = lines.next().unwrap().unwrap();
    if s.chars()
        .enumerate()
        .filter(|(i, _)| i + 1 >= l && i + 1 <= r)
        .filter(|(_, c)| *c == 'x')
        .count()
        > 0
    {
        println!("No");
    } else {
        println!("Yes");
    }
}
