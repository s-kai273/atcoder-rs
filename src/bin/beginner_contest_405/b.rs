use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let _n: usize = iter.next().unwrap().parse().unwrap();
    let m: u32 = iter.next().unwrap().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut a_list: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let m_list: Vec<u32> = (1..=m).collect();
    let mut count = 0;
    while m_list.iter().all(|item| a_list.contains(item)) {
        a_list.pop();
        count += 1;
    }

    println!("{}", count);
}
