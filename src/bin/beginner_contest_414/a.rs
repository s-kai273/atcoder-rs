use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let l: u32 = iter.next().unwrap().parse().unwrap();
    let r: u32 = iter.next().unwrap().parse().unwrap();
    let mut answer: u32 = 0;
    (0..n).for_each(|_| {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let x: u32 = iter.next().unwrap().parse().unwrap();
        let y: u32 = iter.next().unwrap().parse().unwrap();
        if x <= l && y >= r {
            answer += 1;
        }
    });
    println!("{}", answer);
}
