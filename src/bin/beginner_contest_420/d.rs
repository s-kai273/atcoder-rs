use std::collections::VecDeque;

fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let h: usize = iter.next().unwrap().parse().unwrap();
    let w: usize = iter.next().unwrap().parse().unwrap();
    let a_list: Vec<Vec<char>> = (0..h)
        .map(|_| lines.next().unwrap().unwrap().chars().collect())
        .collect();
    let mut s: (Option<usize>, Option<usize>) = (None, None);
    let mut g: (Option<usize>, Option<usize>) = (None, None);
    for i in 0..h {
        for j in 0..w {
            if a_list[i][j] == 'S' {
                s = (Some(i), Some(j));
            }
            if a_list[i][j] == 'G' {
                g = (Some(i), Some(j));
            }
        }
    }

    let s: (usize, usize) = (s.0.unwrap(), s.1.unwrap());
    let g: (usize, usize) = (g.0.unwrap(), g.1.unwrap());
    let mut door_switched: bool = false;
    let mut bfs_queue: VecDeque<(usize, usize)> = VecDeque::from([s]);
    let mut count_list: Vec<Vec<i32>> = vec![vec![-1; w]; h];
    let mut count: i32 = 0;

    loop {
        let pos: (usize, usize) = bfs_queue.pop_front().unwrap();
        match a_list[pos.0][pos.1] {
            '.' => {
                if count_list[pos.0][pos.1] == -1 {
                    count_list[pos.0][pos.1] = count;
                }
            }
            'S' => {
                if count_list[pos.0][pos.1] == -1 {
                    count_list[pos.0][pos.1] = count;
                }
            }
            'G' => {
                if count_list[pos.0][pos.1] == -1 {
                    count_list[pos.0][pos.1] = count;
                }
                break;
            }
            'o' => {
                if !door_switched && count_list[pos.0][pos.1] == -1 {
                    count_list[pos.0][pos.1] = count;
                } else {
                    continue;
                }
            }
            'x' => {
                if door_switched && count_list[pos.0][pos.1] == -1 {
                    count_list[pos.0][pos.1] = count;
                } else {
                    continue;
                }
            }
            '?' => {
                if count_list[pos.0][pos.1] == -1 {
                    count_list[pos.0][pos.1] = count;
                }
                door_switched = !door_switched;
            }
            _ => unreachable!("Invalid charactor"),
        }
        for next_pos in [
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ] {
            let ch: char = a_list[next_pos.0 as usize][next_pos.1 as usize];
            match ch {
                '#' => continue,
                'o' => {
                    if door_switched {
                        continue;
                    }
                }
                'x' => {
                    if !door_switched {
                        continue;
                    }
                }
                _ => {}
            }
            bfs_queue.push_back(next_pos);
        }
        count += 1;
    }
    println!("{}", count_list[g.0][g.1]);
}
