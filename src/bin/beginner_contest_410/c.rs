use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let mut a_list: Vec<usize> = (1..=n).collect();
    let mut offset: usize = 0;
    (0..q).for_each(|_| {
        let query: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|q| q.parse().unwrap())
            .collect();
        match query[0] {
            1 => {
                let p = query[1];
                let x = query[2];
                if p > n - offset {
                    a_list[p - (n - offset) - 1] = x;
                } else {
                    a_list[p + offset - 1] = x;
                }
            }
            2 => {
                let p = query[1];
                if p > n - offset {
                    println!("{}", a_list[p - (n - offset) - 1]);
                } else {
                    println!("{}", a_list[p + offset - 1]);
                }
            }
            3 => {
                let k = query[1];
                offset = (offset + k) % n;
            }
            _ => unreachable!("invalid query"),
        }
    });
}
