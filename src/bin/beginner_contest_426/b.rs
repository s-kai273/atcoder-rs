fn main() {
    let mut lines = std::io::stdin().lines();
    let s: Vec<char> = lines.next().unwrap().unwrap().trim().chars().collect();
    let mut answer = s[0];
    for i in 0..s.len() {
        let mut count: u32 = 0;
        for j in 0..s.len() {
            if i == j {
                continue;
            }
            if s[i] == s[j] {
                count += 1;
                break;
            }
        }
        if count == 0 {
            answer = s[i];
            break;
        }
    }
    println!("{}", answer);
}
