use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut state: bool = false;
    let mut answer: u32 = 0;
    (0..n).for_each(|_| {
        let s = lines.next().unwrap().unwrap();
        match s.as_str() {
            "login" => state = true,
            "logout" => state = false,
            "public" => {}
            "private" => {
                if state == false {
                    answer += 1;
                }
            }
            _ => unreachable!("invalid operation!"),
        }
    });
    println!("{}", answer);
}
