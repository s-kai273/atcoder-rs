fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let a: u32 = iter.next().unwrap().parse().unwrap();
    let b: u32 = iter.next().unwrap().parse().unwrap();
    let c: u32 = iter.next().unwrap().parse().unwrap();
    let answer: &str = if a == b || b == c || c == a {
        "Yes"
    } else {
        "No"
    };
    println!("{}", answer);
}
