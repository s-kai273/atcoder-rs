fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let _: usize = iter.next().unwrap().parse().unwrap();
    let a_list: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();
    let mut answer: Vec<usize> = Vec::new();
    for i in 1..n + 1 {
        if !a_list.contains(&i) {
            answer.push(i);
        }
    }
    println!("{}", answer.len());
    println!(
        "{}",
        answer
            .iter()
            .map(|&val| val.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}
