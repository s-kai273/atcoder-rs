fn main() {
    let mut lines = std::io::stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut s_list: Vec<String> = (0..n)
        .map(|_| lines.next().unwrap().unwrap().trim().to_string())
        .collect();
    s_list.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("{}", s_list.join(""));
}
