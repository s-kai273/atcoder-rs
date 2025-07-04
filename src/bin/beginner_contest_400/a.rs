use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let a: u32 = lines.next().unwrap().unwrap().parse().unwrap();
    if 400 % a == 0 {
        println!("{}", 400 / a);
    } else {
        println!("-1");
    }
}
