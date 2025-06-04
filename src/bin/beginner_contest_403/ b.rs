use std::io::BufRead;

fn compare_char(t: char, u: char) -> bool {
    if t == '?' || t == u { true } else { false }
}

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let t_list: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let u_list: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let mut answer = "No";

    for i in 0..t_list.len() - u_list.len() + 1 {
        for j in 0..u_list.len() {
            if !compare_char(t_list[i + j], u_list[j]) {
                break;
            }
            if j == u_list.len() - 1 {
                answer = "Yes";
            }
        }
    }

    println!("{}", answer);
}
