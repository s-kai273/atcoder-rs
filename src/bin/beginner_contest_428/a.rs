fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let s: u32 = iter.next().unwrap().parse().unwrap();
    let a: u32 = iter.next().unwrap().parse().unwrap();
    let b: u32 = iter.next().unwrap().parse().unwrap();
    let x: u32 = iter.next().unwrap().parse().unwrap();
    let mut answer: u32 = 0;
    let mut t: u32 = 0;
    while t < x {
        let time = if x - t > a { a } else { x - t };
        t += time + b;
        answer += s * time;
    }
    println!("{}", answer);
}
