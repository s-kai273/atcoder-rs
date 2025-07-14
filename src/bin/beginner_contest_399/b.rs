use std::{collections::HashMap, io::BufRead};
fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let _: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let p_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|p| p.parse().unwrap())
        .collect();
    let mut p_count_map: HashMap<u32, usize> = HashMap::new();
    p_list.iter().for_each(|p| {
        *p_count_map.entry(*p).or_insert(0) += 1;
    });
    let mut p_r_list: HashMap<u32, usize> = HashMap::new();
    let mut keys: Vec<u32> = p_count_map.keys().cloned().collect();
    keys.sort();
    keys.reverse();
    let mut r: usize = 1;
    keys.iter().for_each(|key| {
        p_r_list.insert(*key, r);
        r += p_count_map.get(key).unwrap();
    });
    p_list.iter().for_each(|p| {
        println!("{}", p_r_list.get(p).unwrap());
    });
}
