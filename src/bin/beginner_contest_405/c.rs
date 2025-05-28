use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let a_list: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let sum: u64 = a_list.iter().sum();
    let mut answer = sum * sum;
    let mut partial_sum = 0;
    for a in a_list {
        partial_sum += a;
        answer -= a * partial_sum;
    }
    println!("{}", answer);
}
