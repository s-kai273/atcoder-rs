fn main() {
    let mut lines = std::io::stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let a_list: Vec<u64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();
    let mut answer: &str = "Yes";
    for i in 0..n - 2 {
        if a_list[i] * a_list[i + 2] != a_list[i + 1] * a_list[i + 1] {
            answer = "No";
            break;
        }
    }
    println!("{}", answer);
}
