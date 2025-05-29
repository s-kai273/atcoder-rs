use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut a_list: Vec<Vec<u32>> = Vec::new();
    for _ in 0..3 {
        a_list.push(
            lines
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
        );
    }

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut marked = [[false; 3]; 3];
    for _ in 0..n {
        let b: u32 = lines.next().unwrap().unwrap().parse().unwrap();
        for i in 0..3 {
            for j in 0..3 {
                if b == a_list[i][j] {
                    marked[i][j] = true;
                }
            }
        }
    }

    let mut result = "No";
    for i in 0..3 {
        if (marked[i][0] && marked[i][1] && marked[i][2])
            || (marked[0][i] && marked[1][i] && marked[2][i])
        {
            result = "Yes";
        }
    }
    if (marked[0][0] && marked[1][1] && marked[2][2])
        || (marked[0][2] && marked[1][1] && marked[2][0])
    {
        result = "Yes";
    }

    println!("{}", result);
}
