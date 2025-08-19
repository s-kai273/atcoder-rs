use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    let mut lines = std::io::stdin().lines();
    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let q_list: Vec<String> = (0..q).map(|_| lines.next().unwrap().unwrap()).collect();
    let mut ball_list: BinaryHeap<Reverse<u32>> = BinaryHeap::new();
    q_list.iter().for_each(|querey: &String| {
        let query_parts: Vec<&str> = querey.split_whitespace().collect();
        match query_parts[0] {
            "1" => {
                let x: u32 = query_parts[1].parse().unwrap();
                ball_list.push(Reverse(x));
            }
            "2" => {
                if let Some(Reverse(x)) = ball_list.pop() {
                    println!("{}", x);
                }
            }
            _ => unreachable!("invalid query"),
        }
    });
}
