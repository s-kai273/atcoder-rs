fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let x: u32 = iter.next().unwrap().parse().unwrap();
    let c: u32 = iter.next().unwrap().parse().unwrap();
    let answer: u32 = (x as f32 / (1.0 + (c as f32) / 1000.0)) as u32;
    let answer: u32 = answer - (answer % 1000);
    println!("{}", answer);
}
