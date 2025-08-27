fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let s_list: Vec<Vec<char>> = (0..n)
        .map(|_| lines.next().unwrap().unwrap().trim().chars().collect())
        .collect();
    let mut score_list: Vec<u32> = vec![0; n];
    for i in 0..m {
        let mut x_count: usize = 0;
        let mut y_count: usize = 0;
        for j in 0..n {
            if s_list[j][i] == '0' {
                x_count += 1;
            } else {
                y_count += 1;
            }
        }

        if x_count == 0 || y_count == 0 {
            for j in 0..n {
                score_list[j] += 1;
            }
        } else if x_count < y_count {
            for j in 0..n {
                if s_list[j][i] == '0' {
                    score_list[j] += 1;
                }
            }
        } else {
            for j in 0..n {
                if s_list[j][i] == '1' {
                    score_list[j] += 1;
                }
            }
        }
    }
    let mut max_score: u32 = 0;
    let mut max_index_list: Vec<usize> = Vec::new();
    score_list.iter().enumerate().for_each(|(index, &score)| {
        if max_score < score {
            max_score = score;
            max_index_list = vec![index + 1];
        } else if max_score == score {
            max_index_list.push(index + 1);
        }
    });
    println!(
        "{}",
        max_index_list
            .iter()
            .map(|index| index.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
