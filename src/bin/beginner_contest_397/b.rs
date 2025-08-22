fn main() {
    let mut lines = std::io::stdin().lines();
    let s: String = lines.next().unwrap().unwrap().trim().to_string();
    let s_chars: Vec<char> = s.chars().collect();
    let mut s_chars_right: Vec<char> = vec![s_chars[0]];
    for i in 1..s.len() {
        if s_chars[i] == s_chars[i - 1] && s_chars[i] == 'i' {
            s_chars_right.push('o');
        } else if s_chars[i] == s_chars[i - 1] && s_chars[i] == 'o' {
            s_chars_right.push('i');
        }
        s_chars_right.push(s_chars[i]);
    }
    if s_chars_right[0] == 'o' {
        s_chars_right.insert(0, 'i');
    }
    if s_chars_right.len() % 2 != 0 {
        s_chars_right.push('o');
    }
    println!("{}", s_chars_right.len() - s_chars.len());
}
