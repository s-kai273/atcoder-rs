use std::{collections::HashSet, io::BufRead, str::SplitWhitespace, vec};

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter: SplitWhitespace = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let _: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let mut auth_list: Vec<HashSet<u32>> = vec![HashSet::new(); n];
    let mut auth_all_list: HashSet<u32> = HashSet::new();
    let mut answer_list: Vec<String> = Vec::new();

    (0..q).into_iter().for_each(|_| {
        let query = lines.next().unwrap().unwrap();
        let query: Vec<u32> = query
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        match query[0] {
            1 => {
                auth_list[(query[1] - 1) as usize].insert(query[2] - 1);
            }
            2 => {
                auth_all_list.insert(query[1] - 1);
            }
            3 => {
                let answer = if auth_all_list.contains(&(query[1] - 1)) {
                    "Yes"
                } else if auth_list[(query[1] - 1) as usize].contains(&(query[2] - 1)) {
                    "Yes"
                } else {
                    "No"
                };
                answer_list.push(answer.to_string());
            }
            _ => unreachable!("invalid query type"),
        }
    });

    answer_list.iter().for_each(|answer| {
        println!("{}", answer);
    });
}
