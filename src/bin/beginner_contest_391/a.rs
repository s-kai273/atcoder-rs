fn main() {
    let mut lines = std::io::stdin().lines();
    let d: Vec<char> = lines.next().unwrap().unwrap().trim().chars().collect();
    let answer: String = d
        .into_iter()
        .map(|ch| {
            if ch == 'N' {
                'S'
            } else if ch == 'S' {
                'N'
            } else if ch == 'E' {
                'W'
            } else {
                'E'
            }
        })
        .collect();
    println!("{}", answer);
}
