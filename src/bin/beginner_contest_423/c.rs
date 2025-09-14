fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let r: usize = iter.next().unwrap().parse().unwrap();
    let l_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|l| l.parse().unwrap())
        .collect();
    let mut min_open_idx: usize = 2 * n;
    let mut max_open_idx: usize = 2 * n;
    for i in 0..n {
        if l_list[i] == 0 {
            min_open_idx = i;
            break;
        }
    }
    for i in (0..n).rev() {
        if l_list[i] == 0 {
            max_open_idx = i;
            break;
        }
    }
    let target_min_idx = if min_open_idx < r { min_open_idx } else { r };
    let target_max_idx = if max_open_idx + 1 > r {
        max_open_idx
    } else {
        r - 1
    };
    let target_l_list = if min_open_idx == 2 * n && max_open_idx == 2 * n {
        &l_list
    } else {
        &l_list[target_min_idx..target_max_idx + 1]
    };
    let answer: usize = if min_open_idx == 2 * n && max_open_idx == 2 * n {
        0
    } else {
        target_l_list
            .iter()
            .map(|l| if *l == 0 { 1 } else { 2 })
            .sum()
    };
    println!("{}", answer);
}
