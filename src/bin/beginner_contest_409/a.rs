use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let t_list: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let a_list: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let answer = if (0..n).any(|i| t_list[i] == 'o' && a_list[i] == 'o') {
        "Yes"
    } else {
        "No"
    };
    println!("{}", answer);
}
