use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let _: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a_excluded_set: HashSet<u32> = HashSet::new();
    let mut a_unique_map: HashMap<u32, usize> = HashMap::new();
    lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .for_each(|(i, a)| {
            let a: u32 = a.parse().unwrap();
            if !a_excluded_set.contains(&a) && !a_unique_map.contains_key(&a) {
                a_unique_map.insert(a, i + 1);
            } else if !a_excluded_set.contains(&a) && a_unique_map.contains_key(&a) {
                a_unique_map.remove(&a);
                a_excluded_set.insert(a);
            }
        });
    let mut a_unique_list: Vec<u32> = a_unique_map.keys().cloned().collect();
    a_unique_list.sort();
    let answer: i64 = if a_unique_list.len() > 0 {
        a_unique_map[&a_unique_list[a_unique_list.len() - 1]] as i64
    } else {
        -1
    };
    println!("{}", answer);
}
