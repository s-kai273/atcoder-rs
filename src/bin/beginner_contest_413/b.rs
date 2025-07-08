use std::{collections::HashSet, io::BufRead};

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let s_list: Vec<String> = (0..n).map(|_| lines.next().unwrap().unwrap()).collect();
    let mut com_list: HashSet<String> = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if i != j {
                com_list.insert(format!("{}{}", s_list[i], s_list[j]));
            }
        }
    }
    println!("{}", com_list.len());
}
