fn main() {
    let mut lines = std::io::stdin().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let test_cases: Vec<Vec<usize>> = (0..t)
        .map(|_| {
            lines
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    test_cases.iter().for_each(|case| {
        let na: usize = case[0];
        let nb: usize = case[1];
        let nc: usize = case[2];
        let answer: usize = na.min(nc).min((na + nb + nc) / 3);
        println!("{}", answer);
    });
}
