use std::cmp;

fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let mut a_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();
    let mut b_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|b| b.parse().unwrap())
        .collect();
    let mut min_list: Vec<u32> = (0..n).map(|i| cmp::min(a_list[i], b_list[i])).collect();
    let mut min_sum: u64 = min_list.iter().map(|&value| value as u64).sum();
    let query_list: Vec<String> = (0..q)
        .map(|_| lines.next().unwrap().unwrap().trim().to_string())
        .collect();
    query_list.iter().for_each(|query| {
        let query_parts: Vec<&str> = query.split_whitespace().collect();
        match query_parts[0] {
            "A" => {
                let x: usize = query_parts[1].parse().unwrap();
                let v: u32 = query_parts[2].parse().unwrap();
                a_list[x - 1] = v;
                let new_min: u32 = cmp::min(a_list[x - 1], b_list[x - 1]);
                min_sum = min_sum - min_list[x - 1] as u64 + new_min as u64;
                min_list[x - 1] = new_min;
                println!("{}", min_sum);
            }
            "B" => {
                let x: usize = query_parts[1].parse().unwrap();
                let v: u32 = query_parts[2].parse().unwrap();
                b_list[x - 1] = v;
                let new_min: u32 = cmp::min(a_list[x - 1], b_list[x - 1]);
                min_sum = min_sum - min_list[x - 1] as u64 + new_min as u64;
                min_list[x - 1] = new_min;
                println!("{}", min_sum);
            }
            _ => unreachable!("invalid query"),
        }
    });
}
