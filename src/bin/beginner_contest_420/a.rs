fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let x: u32 = iter.next().unwrap().parse().unwrap();
    let y: u32 = iter.next().unwrap().parse().unwrap();
    let answer: u32 = if x + y <= 12 { x + y } else { x + y - 12 };
    println!("{}", answer);
}
