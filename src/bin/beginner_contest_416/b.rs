fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut t: Vec<char> = s.trim().chars().collect();
    let mut got_sharp: bool = true;
    for i in 0..t.len() {
        if t[i] == '#' {
            got_sharp = true;
        } else if t[i] == '.' && got_sharp {
            t[i] = 'o';
            got_sharp = false;
        }
    }
    println!("{}", t.iter().collect::<String>());
}
