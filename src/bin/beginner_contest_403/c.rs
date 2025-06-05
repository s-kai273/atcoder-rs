use std::{io::BufRead, str::SplitWhitespace, vec};

#[derive(Clone)]
enum AuthType {
    All,
    Pages(Vec<u32>),
    None,
}

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter: SplitWhitespace = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let _: usize = iter.next().unwrap().parse().unwrap();
    let q: u32 = iter.next().unwrap().parse().unwrap();
    let mut auth_list: Vec<AuthType> = vec![AuthType::None; n];
    let mut answer_list: Vec<String> = Vec::new();

    (0..q).into_iter().for_each(|_| {
        let query = lines.next().unwrap().unwrap();
        let query: Vec<usize> = query
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        match query[0] {
            1 => match auth_list[query[1] - 1] {
                AuthType::Pages(ref mut pages) => {
                    if !pages.contains(&(query[2] as u32)) {
                        pages.push(query[2] as u32);
                    }
                }
                AuthType::All => {}
                AuthType::None => {
                    auth_list[query[1] - 1] = AuthType::Pages(vec![query[2] as u32]);
                }
            },
            2 => {
                auth_list[query[1] - 1] = AuthType::All;
            }
            3 => {
                let answer = match &auth_list[query[1] - 1] {
                    AuthType::All => "Yes",
                    AuthType::Pages(pages) => {
                        if pages.contains(&(query[2] as u32)) {
                            "Yes"
                        } else {
                            "No"
                        }
                    }
                    AuthType::None => "No",
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
