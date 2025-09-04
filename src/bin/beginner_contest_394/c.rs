fn main() {
    let mut lines = std::io::stdin().lines();
    let s: String = lines.next().unwrap().unwrap().trim().to_string();
    let mut new_s = String::new();
    let mut successive_w_count: usize = 0;
    for ch in s.chars() {
        if ch == 'W' {
            successive_w_count += 1;
        } else if ch == 'A' {
            if successive_w_count > 0 {
                new_s.push('A');
                new_s.push_str("C".repeat(successive_w_count).as_str());
            } else {
                new_s.push('A');
            }
            successive_w_count = 0;
        } else {
            if successive_w_count > 0 {
                new_s.push_str("W".repeat(successive_w_count).as_str());
            }
            new_s.push(ch);
            successive_w_count = 0;
        }
    }
    if successive_w_count > 0 {
        new_s.push_str("W".repeat(successive_w_count).as_str());
    }
    println!("{}", new_s);
}
