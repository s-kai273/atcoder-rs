fn dfs(
    i: isize,
    j: isize,
    h: isize,
    w: isize,
    g: (isize, isize),
    c_list: &Vec<Vec<char>>,
    visited_list: &mut Vec<Vec<bool>>,
) -> bool {
    visited_list[i as usize][j as usize] = true;
    for current in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
        if current.0 == g.0 && current.1 == g.1 {
            return true;
        }
        if current.0 >= 0
            && current.0 < h
            && current.1 >= 0
            && current.1 < w
            && !visited_list[current.0 as usize][current.1 as usize]
            && c_list[current.0 as usize][current.1 as usize] != '#'
        {
            let res = dfs(current.0, current.1, h, w, g, c_list, visited_list);
            if res {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let h: isize = iter.next().unwrap().parse().unwrap();
    let w: isize = iter.next().unwrap().parse().unwrap();
    let c_list: Vec<Vec<char>> = (0..h)
        .map(|_| lines.next().unwrap().unwrap().chars().collect())
        .collect();
    let mut s: Option<(isize, isize)> = None;
    let mut g: Option<(isize, isize)> = None;
    for i in 0..h {
        for j in 0..w {
            if c_list[i as usize][j as usize] == 's' {
                s = Some((i, j));
            } else if c_list[i as usize][j as usize] == 'g' {
                g = Some((i, j));
            }
        }
    }

    let s = s.unwrap();
    let g = g.unwrap();

    let mut visited_list: Vec<Vec<bool>> = vec![vec![false; w as usize]; h as usize];
    let result = dfs(s.0, s.1, h, w, g, &c_list, &mut visited_list);
    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}
