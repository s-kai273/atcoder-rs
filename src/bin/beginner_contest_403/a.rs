fn main() {
    let mut lines = std::io::stdin().lines();
    let _: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let a_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();
    let answer: u32 = a_list
        .iter()
        .enumerate()
        .filter(|(index, _)| index % 2 == 0)
        .map(|(_, a)| *a)
        .sum();
    println!("{}", answer);
}
