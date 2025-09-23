fn main() {
    let mut lines = std::io::stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let s: Vec<char> = lines.next().unwrap().unwrap().trim().chars().collect();
    let a_pos_list: Vec<usize> = s
        .iter()
        .enumerate()
        .filter_map(|(i, &ch)| if ch == 'A' { Some(i) } else { None })
        .collect();
    let mut ab_cost: u64 = 0;
    for i in 0..n {
        ab_cost += (2 * i as isize - a_pos_list[i] as isize).abs() as u64;
    }
    let mut ba_cost: u64 = 0;
    for i in 0..n {
        ba_cost += (2 * i as isize + 1 - a_pos_list[i] as isize).abs() as u64;
    }
    let answer = if ab_cost > ba_cost { ba_cost } else { ab_cost };
    println!("{}", answer);
}
