use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let _: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let a_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();
    let x: u32 = lines.next().unwrap().unwrap().parse().unwrap();
    if a_list.contains(&x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
