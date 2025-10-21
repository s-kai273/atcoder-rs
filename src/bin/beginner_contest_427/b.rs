fn main() {
    let mut lines = std::io::stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut answer: u32 = 0;
    let mut a_list: Vec<u32> = Vec::new();
    for i in 0..n {
        if i == 0 {
            a_list.push(1);
            answer += 1;
            continue;
        }
        answer += (a_list[i - 1] % 10000) / 1000
            + (a_list[i - 1] % 1000) / 100
            + (a_list[i - 1] % 100) / 10
            + a_list[i - 1] % 10;
        a_list.push(answer);
    }
    println!("{}", answer);
}
