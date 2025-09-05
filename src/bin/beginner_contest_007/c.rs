use std::collections::VecDeque;

fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let r: usize = iter.next().unwrap().parse().unwrap();
    let c: usize = iter.next().unwrap().parse().unwrap();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let sx: usize = iter.next().unwrap().parse().unwrap();
    let sy: usize = iter.next().unwrap().parse().unwrap();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let gx: usize = iter.next().unwrap().parse().unwrap();
    let gy: usize = iter.next().unwrap().parse().unwrap();
    let mut s_list: Vec<Vec<char>> = Vec::new();
    for _ in 0..r {
        s_list.push(lines.next().unwrap().unwrap().chars().collect());
    }
    let mut queue: VecDeque<(usize, usize)> = VecDeque::from([(sx - 1, sy - 1)]);
    let mut dist_list: Vec<Vec<i32>> = vec![vec![-1; c]; r];
    dist_list[sx - 1][sy - 1] = 0;
    'outer: loop {
        let pos = queue.pop_front().unwrap();
        for next_pos in vec![
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ] {
            if s_list[next_pos.0][next_pos.1] == '.' && dist_list[next_pos.0][next_pos.1] == -1 {
                dist_list[next_pos.0][next_pos.1] = dist_list[pos.0][pos.1] + 1;
                queue.push_back(next_pos);
                if next_pos.0 == gx - 1 && next_pos.1 == gy - 1 {
                    break 'outer;
                }
            }
        }
    }
    println!("{}", dist_list[gx - 1][gy - 1]);
}
