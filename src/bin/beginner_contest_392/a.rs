fn main() {
    let mut lines = std::io::stdin().lines();
    let a_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();
    let answer = if a_list[0] * a_list[1] == a_list[2]
        || a_list[1] * a_list[2] == a_list[0]
        || a_list[2] * a_list[0] == a_list[1]
    {
        "Yes"
    } else {
        "No"
    };
    println!("{}", answer);
}
