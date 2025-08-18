fn main() {
    let mut lines = std::io::stdin().lines();
    let s: String = lines.next().unwrap().unwrap().trim().to_string();
    let answer: String = if s == "red" {
        "SSS".to_string()
    } else if s == "blue" {
        "FFF".to_string()
    } else if s == "green" {
        "MMM".to_string()
    } else {
        "Unknown".to_string()
    };
    println!("{}", answer);
}
