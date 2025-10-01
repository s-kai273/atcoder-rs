fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let a_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();
    let query_list: Vec<Vec<u32>> = (0..q)
        .map(|_| {
            lines
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|q| q.parse().unwrap())
                .collect()
        })
        .collect();
    let mut sum_list: Vec<u32> = Vec::from([0]);
    for i in 1..2 * (n + 1) {
        sum_list.push(sum_list[i - 1] + a_list[(i - 1) % n]);
    }
    let mut head: usize = 0;
    query_list.iter().for_each(|query| match query[0] {
        1 => {
            let c = query[1] as usize;
            head += c % n;
        }
        2 => {
            let l = query[1] as usize;
            let r = query[2] as usize;
            let sum: u32 = sum_list[head + r] - sum_list[head + l - 1];
            println!("{}", sum);
        }
        _ => unreachable!(),
    });
}
