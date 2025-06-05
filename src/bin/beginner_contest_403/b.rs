use std::io::BufRead;

fn compare_char(t: char, u: char) -> bool {
    t == '?' || t == u
}

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let t_list: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let u_list: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let result = t_list.windows(u_list.len()).any(|window| {
        window
            .iter()
            .zip(&u_list)
            .all(|(&t, &u)| compare_char(t, u))
    });
    let answer = if result { "Yes" } else { "No" };
    println!("{}", answer);
}
