fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let _: usize = iter.next().unwrap().parse().unwrap();
    let _: usize = iter.next().unwrap().parse().unwrap();
    let mut a_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();
    lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .for_each(|b| {
            let b: u32 = b.parse().unwrap();
            if let Some(pos) = a_list.iter().position(|&a| a == b) {
                a_list.remove(pos);
            }
        });
    println!(
        "{}",
        a_list
            .iter()
            .map(|&a| a.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
