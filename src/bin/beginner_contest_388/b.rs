fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let d: usize = iter.next().unwrap().parse().unwrap();
    let tl_list: Vec<(u32, u32)> = (0..n)
        .map(|_| {
            let line = lines.next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            let t: u32 = iter.next().unwrap().parse().unwrap();
            let l: u32 = iter.next().unwrap().parse().unwrap();
            (t, l)
        })
        .collect();
    for k in 1..d + 1 {
        let mut max_w: u32 = 0;
        for (t, l) in &tl_list {
            if t * (l + k as u32) > max_w {
                max_w = t * (l + k as u32);
            }
        }
        println!("{}", max_w);
    }
}
