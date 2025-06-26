fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s: u32 = s.trim().parse().unwrap();
    let answer = if s >= 200 && s <= 299 {
        "Success"
    } else {
        "Failure"
    };
    println!("{}", answer);
}
