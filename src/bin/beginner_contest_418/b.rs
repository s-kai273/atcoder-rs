fn main() {
    let mut lines = std::io::stdin().lines();
    let s = lines.next().unwrap().unwrap().trim().to_string();
    let s_chars: Vec<char> = s.chars().collect();
    let mut answer: f64 = 0.0;
    for i in 0..s_chars.len() {
        if s_chars[i] != 't' {
            continue;
        }
        let mut t_count: usize = 1;
        for j in i + 1..s_chars.len() {
            if s_chars[j] != 't' {
                continue;
            }
            t_count += 1;
            if t_count >= 3 {
                let load_factor: f64 = (t_count - 2) as f64 / (j - (i - 1) - 2) as f64;
                if load_factor > answer {
                    answer = load_factor;
                }
            }
        }
    }
    println!("{:.16}", answer);
}
