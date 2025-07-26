use std::{collections::HashMap, io::BufRead};

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let mut num_count_map: HashMap<u32, usize> = HashMap::new();
    lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .for_each(|a| {
            let a: u32 = a.parse().unwrap();
            *num_count_map.entry(a).or_insert(0) += 1;
        });
    let answer: String = if num_count_map
        .iter()
        .filter(|(_key, value)| **value >= 3)
        .count()
        >= 1
        && num_count_map
            .iter()
            .filter(|(_key, value)| **value >= 2)
            .count()
            >= 2
    {
        "Yes".to_string()
    } else {
        "No".to_string()
    };
    println!("{}", answer);
}
