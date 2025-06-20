use std::{collections::VecDeque, io::BufRead};

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut queue: VecDeque<u32> = VecDeque::new();
    (0..q).for_each(|_| {
        let query: Vec<u32> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        match query[0] {
            1 => {
                queue.push_back(query[1]);
            }
            2 => {
                let menu = queue.pop_front().unwrap();
                println!("{}", menu);
            }
            _ => unreachable!("invalid query"),
        }
    });
}
