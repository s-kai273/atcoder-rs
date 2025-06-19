fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let answer: String = s.chars().filter(|ch| ch.is_uppercase()).collect();
    println!("{}", answer);
}
