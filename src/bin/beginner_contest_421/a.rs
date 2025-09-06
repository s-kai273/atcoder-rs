fn main() {
    let mut lines = std::io::stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let s_list: Vec<String> = (0..n)
        .map(|_| lines.next().unwrap().unwrap().trim().to_ascii_lowercase())
        .collect();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let x: usize = iter.next().unwrap().parse().unwrap();
    let y: String = iter.next().unwrap().trim().to_string();
    let answer = if s_list[x - 1] == y { "Yes" } else { "No" };
    println!("{}", answer);
}
