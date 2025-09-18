fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let s1: String = iter.next().unwrap().trim().to_string();
    let s2: String = iter.next().unwrap().trim().to_string();
    let answer: u32 = if s1 == "sick" && s2 == "sick" {
        1
    } else if s1 == "sick" && s2 == "fine" {
        2
    } else if s1 == "fine" && s2 == "sick" {
        3
    } else {
        4
    };
    println!("{}", answer);
}
