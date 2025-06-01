use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    for c in b'a'..=b'z' {
        let ch = c as char;
        if !s.contains(ch) {
            println!("{}", ch);
            break;
        }
    }
}
