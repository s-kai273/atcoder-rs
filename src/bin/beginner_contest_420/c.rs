use std::{cmp, collections::HashMap};

fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let a_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();
    let b_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|b| b.parse().unwrap())
        .collect();
    let mut a_change_map: HashMap<usize, u32> = HashMap::new();
    let mut b_change_map: HashMap<usize, u32> = HashMap::new();
    (0..q).for_each(|_| {
        let line = lines.next().unwrap().unwrap();
        let query_parts: Vec<&str> = line.split_whitespace().collect();
        match query_parts[0] {
            "A" => {
                let x: usize = query_parts[1].parse().unwrap();
                let v: u32 = query_parts[2].parse().unwrap();
                a_change_map.insert(x, v);
            }
            "B" => {
                let x: usize = query_parts[1].parse().unwrap();
                let v: u32 = query_parts[2].parse().unwrap();
                b_change_map.insert(x, v);
            }
            _ => unreachable!("invalid query"),
        }
    });
    let answer: u32 = (0..n)
        .map(|i| {
            let a: u32 = if let Some(&value) = a_change_map.get(&i) {
                value
            } else {
                a_list[i]
            };
            let b: u32 = if let Some(&value) = b_change_map.get(&i) {
                value
            } else {
                b_list[i]
            };
            cmp::min(a, b)
        })
        .sum();
    println!("{}", answer);
}

