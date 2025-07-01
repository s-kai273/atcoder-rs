use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let s = lines.next().unwrap().unwrap();
    let s_chars: Vec<char> = s.chars().collect();
    let t = lines.next().unwrap().unwrap();
    let mut answer = "Yes";
    (1..s_chars.len()).for_each(|i| {
        if s_chars[i].is_uppercase() {
            if !t.contains(s_chars[i - 1]) {
                answer = "No";
            }
        }
    });
    println!("{}", answer);
}
