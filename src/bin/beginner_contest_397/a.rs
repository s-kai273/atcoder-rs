fn main() {
    let mut lines = std::io::stdin().lines();
    let x: f32 = lines.next().unwrap().unwrap().parse().unwrap();
    let answer: u32 = if x >= 38.0 {
        1
    } else if x >= 37.5 {
        2
    } else {
        3
    };
    println!("{}", answer);
}
