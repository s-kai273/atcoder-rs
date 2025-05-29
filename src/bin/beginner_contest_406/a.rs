use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a: u32 = iter.next().unwrap().parse().unwrap();
    let b: u32 = iter.next().unwrap().parse().unwrap();
    let c: u32 = iter.next().unwrap().parse().unwrap();
    let d: u32 = iter.next().unwrap().parse().unwrap();
    let answer = if a > c || (a == c && b >= d) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", answer)
}
