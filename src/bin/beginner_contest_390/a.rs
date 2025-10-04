fn main() {
    let mut lines = std::io::stdin().lines();
    let a_list: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();
    let mut answer = "No";
    for i in 0..a_list.len() - 1 {
        let mut swaped_a_list = a_list.clone();
        swaped_a_list.swap(i, i + 1);
        if swaped_a_list == vec![1, 2, 3, 4, 5] {
            answer = "Yes";
            break;
        }
    }
    println!("{}", answer);
}
