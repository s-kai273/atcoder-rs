fn main() {
    let mut lines = std::io::stdin().lines();
    let _: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let s: String = lines.next().unwrap().unwrap().trim().to_string();
    if s.ends_with("tea") {
        println!("Yes");
    } else {
        println!("No");
    }
}
