use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let equal_index_list: Vec<usize> = if n % 2 == 0 {
        vec![n / 2 - 1, n / 2]
    } else {
        vec![n / 2]
    };
    let mut answer: String = String::new();
    for i in 0..n {
        if equal_index_list.contains(&i) {
            answer.push_str("=");
        } else {
            answer.push_str("-");
        }
    }
    println!("{}", answer);
}
