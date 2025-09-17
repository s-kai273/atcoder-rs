fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let h: usize = iter.next().unwrap().parse().unwrap();
    let w: usize = iter.next().unwrap().parse().unwrap();
    let s_list: Vec<Vec<char>> = (0..h)
        .map(|_| lines.next().unwrap().unwrap().chars().collect())
        .collect();
    let mut answer: &str = "Yes";
    'outer: for i in 0..h {
        for j in 0..w {
            if s_list[i][j] == '#' {
                let mut black_count: usize = 0;
                for xy in [
                    (i as isize, j as isize + 1),
                    (i as isize, j as isize - 1),
                    (i as isize + 1, j as isize),
                    (i as isize - 1, j as isize),
                ] {
                    if xy.0 < 0 || xy.0 > h as isize - 1 || xy.1 < 0 || xy.1 > w as isize - 1 {
                        continue;
                    }
                    if s_list[xy.0 as usize][xy.1 as usize] == '#' {
                        black_count += 1;
                    }
                }
                if black_count != 2 && black_count != 4 {
                    answer = "No";
                    break 'outer;
                }
            }
        }
    }
    println!("{}", answer);
}
