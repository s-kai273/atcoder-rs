fn main() {
    let mut lines = std::io::stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let answer: i32 = (1..n + 1)
        .map(|i| (-1 as i32).pow(i as u32) * (i as i32).pow(3))
        .sum();
    println!("{}", answer);
}
