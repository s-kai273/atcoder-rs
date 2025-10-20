fn main() {
    let mut lines = std::io::stdin().lines();
    let s: Vec<char> = lines.next().unwrap().unwrap().trim().chars().collect();
    let answer: String = s
        .iter()
        .enumerate()
        .filter_map(|(i, ch)| {
            if i == (s.len() + 1) / 2 - 1 {
                return None;
            }
            Some(ch)
        })
        .collect();
    println!("{}", answer);
}
