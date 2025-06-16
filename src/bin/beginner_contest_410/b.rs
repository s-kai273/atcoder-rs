use std::{io::BufRead, str::SplitWhitespace};

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter: SplitWhitespace = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let mut box_list: Vec<Vec<u32>> = vec![Vec::new(); n];
    let mut answer_list: Vec<i32> = vec![-1; q];
    lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .for_each(|(i, x)| {
            let x: u32 = x.parse().unwrap();
            match x {
                0 => {
                    let (index, _) = box_list
                        .iter()
                        .enumerate()
                        .min_by_key(|(_, b)| b.len())
                        .unwrap();
                    box_list[index].push(i as u32);
                    answer_list[i] = (index + 1) as i32;
                }
                x => {
                    box_list[(x - 1) as usize].push(i as u32);
                    answer_list[i] = x as i32;
                }
            }
        });
    println!(
        "{}",
        answer_list
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
