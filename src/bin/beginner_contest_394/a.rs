fn main() {
    let mut lines = std::io::stdin().lines();
    let s: String = lines.next().unwrap().unwrap().trim().to_string();
    let answer: String = s.chars().filter(|&c| c == '2').collect();
    println!("{}", answer);
}
