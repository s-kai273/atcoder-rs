use std::collections::{HashMap, VecDeque};

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
            } else if a_list[i][j] == 'G' {
                g = (Some(i), Some(j));
            }
        }
    }

    let s: (usize, usize) = (s.0.unwrap(), s.1.unwrap());
    let g: (usize, usize) = (g.0.unwrap(), g.1.unwrap());
    let mut bfs_queue: VecDeque<(bool, usize, usize)> = VecDeque::from([(false, s.0, s.1)]);
    let mut count_list: HashMap<bool, Vec<Vec<i32>>> =
        HashMap::from([(true, vec![vec![-1; w]; h]), (false, vec![vec![-1; w]; h])]);

    count_list.get_mut(&false).unwrap()[s.0][s.1] = 0;
    while bfs_queue.len() > 0 {
        let pos: (bool, usize, usize) = bfs_queue.pop_front().unwrap();
        let door_switched = if a_list[pos.1][pos.2] == '?' {
            true
        } else {
            false
        };
        for next_pos in [
            (
                pos.0 ^ door_switched,
                (pos.1 as isize) + 1,
                (pos.2 as isize),
            ),
            (
                pos.0 ^ door_switched,
                (pos.1 as isize) - 1,
                (pos.2 as isize),
            ),
            (
                pos.0 ^ door_switched,
                (pos.1 as isize),
                (pos.2 as isize) + 1,
            ),
            (
                pos.0 ^ door_switched,
                (pos.1 as isize),
                (pos.2 as isize) - 1,
            ),
        ] {
            if next_pos.1 < 0
                || next_pos.1 >= h as isize
                || next_pos.2 < 0
                || next_pos.2 >= w as isize
            {
                continue;
            }
            let next_pos: (bool, usize, usize) =
                (next_pos.0, next_pos.1 as usize, next_pos.2 as usize);
            if count_list.get(&next_pos.0).unwrap()[next_pos.1][next_pos.2] != -1 {
                continue;
            }
            let ch: char = a_list[next_pos.1 as usize][next_pos.2 as usize];
            match ch {
                '.' => {
                    if count_list.get(&next_pos.0).unwrap()[next_pos.1][next_pos.2] == -1 {
                        count_list.get_mut(&next_pos.0).unwrap()[next_pos.1][next_pos.2] =
                            count_list.get(&pos.0).unwrap()[pos.1][pos.2] + 1;
                    }
                }
                '#' => continue,
                'S' => {
                    if count_list.get(&next_pos.0).unwrap()[next_pos.1][next_pos.2] == -1 {
                        count_list.get_mut(&next_pos.0).unwrap()[next_pos.1][next_pos.2] =
                            count_list.get(&pos.0).unwrap()[pos.1][pos.2] + 1;
                    }
                }
                'G' => {
                    if count_list.get(&next_pos.0).unwrap()[next_pos.1][next_pos.2] == -1 {
                        count_list.get_mut(&true).unwrap()[next_pos.1][next_pos.2] =
                            count_list.get(&pos.0).unwrap()[pos.1][pos.2] + 1;
                        count_list.get_mut(&false).unwrap()[next_pos.1][next_pos.2] =
                            count_list.get(&pos.0).unwrap()[pos.1][pos.2] + 1;
                    }
                    break;
                }
                'o' => {
                    if !next_pos.0
                        && count_list.get(&next_pos.0).unwrap()[next_pos.1][next_pos.2] == -1
                    {
                        count_list.get_mut(&next_pos.0).unwrap()[next_pos.1][next_pos.2] =
                            count_list.get(&pos.0).unwrap()[pos.1][pos.2] + 1;
                    } else {
                        continue;
                    }
                }
                'x' => {
                    if next_pos.0
                        && count_list.get(&next_pos.0).unwrap()[next_pos.1][next_pos.2] == -1
                    {
                        count_list.get_mut(&next_pos.0).unwrap()[next_pos.1][next_pos.2] =
                            count_list.get(&pos.0).unwrap()[pos.1][pos.2] + 1;
                    } else {
                        continue;
                    }
                }
                '?' => {
                    if count_list.get(&next_pos.0).unwrap()[next_pos.1][next_pos.2] == -1 {
                        count_list.get_mut(&next_pos.0).unwrap()[next_pos.1][next_pos.2] =
                            count_list.get(&pos.0).unwrap()[pos.1][pos.2] + 1;
                    }
                }
                _ => {}
            }
            bfs_queue.push_back(next_pos);
        }
    }
    println!("{}", count_list.get(&true).unwrap()[g.0][g.1]);
}
