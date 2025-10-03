use std::collections::HashMap;

fn main() {
    let mut lines = std::io::stdin().lines();
    let _n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let a_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();
    let mut min_count: usize = usize::MAX;
    let mut number_pos_list: HashMap<u32, usize> = HashMap::new();
    a_list.iter().enumerate().for_each(|(i, a)| {
        if let Some(pos) = number_pos_list.get(a) {
            if min_count > i - pos {
                min_count = i - pos;
            }
        }
        number_pos_list.insert(*a, i);
    });
    let answer: String = if min_count == usize::MAX {
        "-1".to_string()
    } else {
        (min_count + 1).to_string()
    };
    println!("{}", answer);
}
