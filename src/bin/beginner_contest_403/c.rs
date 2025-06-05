use std::{io::BufRead, str::SplitWhitespace, vec};

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter: SplitWhitespace = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let q: u32 = iter.next().unwrap().parse().unwrap();
    let mut auth_list: Vec<Vec<bool>> = vec![vec![false; m]; n];
    let query_list: Vec<Vec<usize>> = (0..q)
        .map(|_| {
            lines
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    query_list.iter().for_each(|query| match query[0] {
        1 => {
            auth_list[query[1] - 1][query[2] - 1] = true;
        }
        2 => {
            auth_list[query[1] - 1] = vec![true; m];
        }
        3 => {
            let answer = if auth_list[query[1] - 1][query[2] - 1] {
                "Yes"
            } else {
                "No"
            };
            println!("{}", answer);
        }
        _ => unreachable!("invalid query"),
    });
}
