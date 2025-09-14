fn main() {
    let mut lines = std::io::stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let l_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|l| l.parse().unwrap())
        .collect();
    let mut count: usize = 2;
    for i in 0..n {
        if l_list[i] == 1 {
            break;
        }
        count += 1;
    }
    for i in (0..n).rev() {
        if l_list[i] == 1 {
            break;
        }
        count += 1;
    }
    let answer: usize = if count >= n + 1 { 0 } else { n + 1 - count };
    println!("{}", answer);
}
