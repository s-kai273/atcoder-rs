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
    (0..n).for_each(|i| {
        let mut answer: Vec<u32> = Vec::new();
        (i + 1..n).for_each(|j| {
            answer.push(d_list[i..j].iter().sum());
        });
        println!(
            "{}",
            answer
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    });
}
