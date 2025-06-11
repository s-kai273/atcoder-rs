use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let a_list: Vec<u64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();
    let mut answer = 0;
    for i in 0..n {
        let count = a_list.iter().filter(|&a| *a >= (i + 1) as u64).count();
        if i + 1 <= count {
            answer = i + 1;
        } else {
            break;
        }
    }
    println!("{}", answer);
}
