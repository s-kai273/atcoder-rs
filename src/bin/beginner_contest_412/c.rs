use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut answer: Vec<i32> = Vec::new();
    (0..t).for_each(|_| {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut s_list: Vec<u32> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut s_parts: Vec<u32> = s_list[1..n - 1].to_vec();
        s_parts.sort();
        s_list.splice(1..n - 1, s_parts);
        let mut count: usize = 2;
        let mut prev_i: usize = 0;
        if s_list[0] * 2 >= s_list[n - 1] {
            answer.push(2);
            return;
        }
        for i in 1..n - 1 {
            if s_list[prev_i] * 2 >= s_list[i] && s_list[prev_i] * 2 < s_list[i + 1] {
                prev_i = i;
                count += 1;
                if s_list[prev_i] * 2 >= s_list[n - 1] {
                    answer.push(count as i32);
                    return;
                }
            } else if i == n - 1 && s_list[prev_i] * 2 >= s_list[i] {
                count += 1;
                answer.push(count as i32);
                return;
            } else if s_list[prev_i] * 2 < s_list[i] {
                answer.push(-1);
                return;
            }
        }
        answer.push(-1);
    });
    answer.iter().for_each(|a| println!("{}", a));
}
