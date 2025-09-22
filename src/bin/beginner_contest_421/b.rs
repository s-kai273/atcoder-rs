fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let x: u64 = iter.next().unwrap().parse().unwrap();
    let y: u64 = iter.next().unwrap().parse().unwrap();
    let mut a_list: Vec<u64> = Vec::from([x, y]);
    for i in 2..10 as usize {
        let a_str: String = (a_list[i - 1] + a_list[i - 2]).to_string();
        let a_rev_str: String = a_str.chars().rev().collect();
        let a_rev: u64 = a_rev_str.parse().unwrap();
        a_list.push(a_rev);
    }
    println!("{}", a_list[9]);
}
