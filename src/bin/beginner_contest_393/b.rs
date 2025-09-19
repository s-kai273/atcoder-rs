fn main() {
    let mut lines = std::io::stdin().lines();
    let s: Vec<char> = lines.next().unwrap().unwrap().trim().chars().collect();
    let mut answer: usize = 0;
    for i in 0..s.len() - 1 {
        if s[i] != 'A' {
            continue;
        }
        for j in i + 1..s.len() {
            if j + (j - i) >= s.len() {
                break;
            }
            if s[j] == 'B' && s[j + (j - i)] == 'C' {
                answer += 1;
            }
        }
    }
    println!("{}", answer);
}
