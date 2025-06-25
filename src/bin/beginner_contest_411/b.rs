use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let d_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|d| d.parse().unwrap())
        .collect();
    (0..n - 1).for_each(|i| {
        let answer: Vec<String> = (i + 1..n)
            .map(|j| d_list[i..j].iter().sum::<u32>().to_string())
            .collect();
        println!("{}", answer.join(" "));
    });
}
