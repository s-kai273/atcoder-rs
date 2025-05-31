use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let _: u32 = iter.next().unwrap().parse().unwrap();
    let k: u32 = iter.next().unwrap().parse().unwrap();
    let a_list: Vec<u128> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut answer: u128 = 1;
    for a in a_list {
        answer *= a;
        if answer >= 10u128.pow(k) {
            answer = 1;
        }
    }
    println!("{}", answer);
}
