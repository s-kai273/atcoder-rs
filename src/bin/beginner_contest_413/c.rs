use std::{collections::VecDeque, io::BufRead};

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let mut a: VecDeque<u32> = VecDeque::new();
    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();
    (0..q).for_each(|_| {
        let query: Vec<u32> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        match query[0] {
            1 => {
                a.extend(vec![query[2]; query[1] as usize]);
            }
            2 => {
                let sum: u32 = a.drain(..query[1] as usize).sum();
                println!("{}", sum)
            }
            _ => unreachable!("Invalid query"),
        }
    });
}
