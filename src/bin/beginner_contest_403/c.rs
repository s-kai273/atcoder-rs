use std::{collections::HashMap, io::BufRead, str::SplitWhitespace, vec};

#[derive(Clone)]
enum AuthType {
    Page(HashMap<usize, u128>),
    All,
    None,
}

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter: SplitWhitespace = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let _: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let word_size = 128;
    let mut auth_list: Vec<AuthType> = vec![AuthType::None; n];
    let mut answer_list: Vec<String> = Vec::new();

    (0..q).into_iter().for_each(|_| {
        let query = lines.next().unwrap().unwrap();
        let query: Vec<usize> = query
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        match query[0] {
            1 => match &mut auth_list[query[1] - 1] {
                AuthType::None => {
                    let mut auth_pages = HashMap::new();
                    auth_pages.insert(
                        (query[2] + word_size - 1) / word_size - 1,
                        1u128 << (query[2] % word_size),
                    );
                    auth_list[query[1] - 1] = AuthType::Page(auth_pages);
                }
                AuthType::Page(auth_pages) => {
                    match auth_pages.get_mut(&((query[2] + word_size - 1) / word_size - 1)) {
                        Some(page) => {
                            *page |= 1u128 << (query[2] % word_size);
                        }
                        None => {
                            auth_pages.insert(
                                (query[2] + word_size - 1) / word_size - 1,
                                1u128 << (query[2] % word_size),
                            );
                        }
                    }
                }
                AuthType::All => {}
            },
            2 => {
                auth_list[query[1] - 1] = AuthType::All;
            }
            3 => {
                let answer = match &auth_list[query[1] - 1] {
                    AuthType::None => "No",
                    AuthType::Page(auth_pages) => {
                        let page_opt =
                            auth_pages.get(&((query[2] + word_size - 1) / word_size - 1));
                        match page_opt {
                            Some(page) => {
                                if *page & 1u128 << (query[2] % word_size) != 0 {
                                    "Yes"
                                } else {
                                    "No"
                                }
                            }
                            None => "No",
                        }
                    }
                    AuthType::All => "Yes",
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
