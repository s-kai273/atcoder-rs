use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let _: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let a_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();
    let k: u32 = lines.next().unwrap().unwrap().parse().unwrap();
    let answer = a_list.iter().filter(|&a| *a >= k).count();
    println!("{}", answer);
}
