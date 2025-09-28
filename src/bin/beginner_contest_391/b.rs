fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let s_list: Vec<Vec<char>> = (0..n)
        .map(|_| lines.next().unwrap().unwrap().trim().chars().collect())
        .collect();
    let t_list: Vec<Vec<char>> = (0..m)
        .map(|_| lines.next().unwrap().unwrap().trim().chars().collect())
        .collect();
    let mut answer: Option<(usize, usize)> = None;
    'outer: for a in 0..n - m + 1 {
        'inner: for b in 0..n - m + 1 {
            for i in 0..m {
                for j in 0..m {
                    if t_list[i][j] != s_list[i + a][j + b] {
                        continue 'inner;
                    }
                }
            }
            answer = Some((a, b));
            break 'outer;
        }
    }
    let answer: (usize, usize) = answer.unwrap();
    println!("{} {}", answer.0 + 1, answer.1 + 1);
}
