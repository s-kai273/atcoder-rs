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
    let r: f64 = a_list[1] as f64 / a_list[0] as f64;
    let mut answer: &str = "Yes";
    for i in 1..n - 1 {
        if a_list[i + 1] as f64 / a_list[i] as f64 != r {
            answer = "No";
            break;
        }
    }
    println!("{}", answer);
}
