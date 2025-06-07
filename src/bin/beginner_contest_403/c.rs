use std::{io::BufRead, str::SplitWhitespace, vec};

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter: SplitWhitespace = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let _: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let word_size = 128;
    let mut auth_list: Vec<Vec<u128>> =
        vec![vec![0u128; ((q + word_size - 1) / word_size) as usize]; n];
    let mut answer_list: Vec<String> = Vec::new();

    (0..q).into_iter().for_each(|_| {
        let query = lines.next().unwrap().unwrap();
        let query: Vec<usize> = query
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        match query[0] {
            1 => {
                auth_list[query[1] - 1][(query[2] + word_size - 1) / word_size - 1] |=
                    1u128 << (query[2] % word_size);
            }
            2 => {
                auth_list[query[1] - 1] = vec![!0u128; ((q + word_size - 1) / word_size) as usize];
            }
            3 => {
                let answer = if auth_list[query[1] - 1][(query[2] + word_size - 1) / word_size - 1]
                    & 1u128 << (query[2] % word_size)
                    != 0
                {
                    "Yes"
                } else {
                    "No"
                };
                answer_list.push(answer.to_string());
            }
            _ => unreachable!("invalid query"),
        }
    });

    answer_list.iter().for_each(|answer| {
        println!("{}", answer);
    });
}
