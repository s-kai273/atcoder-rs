fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut answer: Vec<char> = Vec::new();
    for ch in s.chars() {
        if ch.is_uppercase() {
            answer.push(ch);
        }
    }
    println!("{}", answer.into_iter().collect::<String>());
}
