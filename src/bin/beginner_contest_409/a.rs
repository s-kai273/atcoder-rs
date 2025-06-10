use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let t_list: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let a_list: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let mut answer = "No";
    (0..n).for_each(|i| {
        if t_list[i] == 'o' && a_list[i] == 'o' {
            answer = "Yes";
        }
    });
    println!("{}", answer);
}
