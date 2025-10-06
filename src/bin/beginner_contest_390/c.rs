fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let h: usize = iter.next().unwrap().parse().unwrap();
    let w: usize = iter.next().unwrap().parse().unwrap();
    let s_list: Vec<Vec<char>> = (0..h)
        .map(|_| lines.next().unwrap().unwrap().chars().collect())
        .collect();
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    let mut min_x: usize = w - 1;
    let mut min_y: usize = h - 1;
    for i in 0..h {
        for j in 0..w {
            if s_list[i][j] == '#' {
                max_x = if max_x < j { j } else { max_x };
                max_y = if max_y < i { i } else { max_y };
                min_x = if min_x > j { j } else { min_x };
                min_y = if min_y > i { i } else { min_y };
            }
        }
    }
    let mut answer = "Yes";
    for i in min_y..max_y + 1 {
        for j in min_x..max_x + 1 {
            if s_list[i][j] == '.' {
                answer = "No";
                break;
            }
        }
    }
    println!("{}", answer);
}
