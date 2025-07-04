use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: u64 = iter.next().unwrap().parse().unwrap();
    let m: u32 = iter.next().unwrap().parse().unwrap();
    let mut sum: u64 = 0;
    for i in 0..m + 1 {
        sum += n.pow(i);
        if sum > 1_000_000_000 {
            break;
        }
    }
    if sum > 1_000_000_000 {
        println!("inf");
    } else {
        println!("{}", sum);
    }
}
