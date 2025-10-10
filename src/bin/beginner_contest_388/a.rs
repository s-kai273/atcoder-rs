fn main() {
    let mut lines = std::io::stdin().lines();
    let s: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    println!("{}UPC", s[0].to_string());
}
