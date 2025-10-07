fn main() {
    let mut lines = std::io::stdin().lines();
    let s: Vec<char> = lines.next().unwrap().unwrap().trim().chars().collect();
    println!(
        "{}",
        (s[0] as u32 - '0' as u32) * (s[2] as u32 - '0' as u32)
    );
}
