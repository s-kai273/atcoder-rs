use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut answer: usize = 0;
    (0..n).for_each(|_| {
        let query: Vec<u32> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if query[0] < query[1] {
            answer += 1;
        }
    });
    println!("{}", answer);
}
