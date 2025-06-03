use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let _: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let a_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();
    let answer: u32 = a_list
        .iter()
        .enumerate()
        .filter_map(|(index, &a)| if index % 2 == 0 { Some(a) } else { None })
        .sum();
    println!("{}", answer);
}
