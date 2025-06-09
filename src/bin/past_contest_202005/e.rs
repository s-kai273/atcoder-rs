use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let mut line_list: Vec<Vec<bool>> = vec![vec![false; n]; n];
    (0..m).for_each(|_| {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let u: usize = iter.next().unwrap().parse().unwrap();
        let v: usize = iter.next().unwrap().parse().unwrap();
        line_list[u - 1][v - 1] = true;
        line_list[v - 1][u - 1] = true;
    });
    let mut c_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let s_list: Vec<Vec<usize>> = (0..q)
        .map(|_| {
            lines
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();
    s_list.iter().for_each(|s| match s[0] {
        1 => {
            println!("{}", c_list[s[1] - 1]);
            line_list[s[1] - 1]
                .iter()
                .enumerate()
                .filter(|(_, line)| **line)
                .for_each(|(i, _)| {
                    c_list[i] = c_list[s[1] - 1];
                });
        }
        2 => {
            println!("{}", c_list[s[1] - 1]);
            c_list[s[1] - 1] = s[2] as u32;
        }
        _ => unreachable!("invalid query type"),
    });
}
