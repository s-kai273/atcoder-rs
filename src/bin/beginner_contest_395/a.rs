fn main() {
    let mut lines = std::io::stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let a_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();
    let mut answer: &str = "Yes";
    for i in 1..n {
        if a_list[i - 1] >= a_list[i] {
            answer = "No";
            break;
        }
    }
    println!("{}", answer);
}
