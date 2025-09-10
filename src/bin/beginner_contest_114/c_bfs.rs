use std::collections::VecDeque;

fn main() {
    let mut lines = std::io::stdin().lines();
    let n: u64 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut answer: usize = 0;
    let mut queue: VecDeque<String> =
        VecDeque::from(["3".to_string(), "5".to_string(), "7".to_string()]);
    loop {
        let value: String = queue.pop_front().unwrap();
        let value_u64: u64 = value.parse().unwrap();
        if value_u64 > n {
            break;
        }
        if value.contains("3") && value.contains("5") && value.contains("7") {
            answer += 1;
        }
        for ch in ['3', '5', '7'] {
            let mut new_value = value.clone();
            new_value.push(ch);
            queue.push_back(new_value);
        }
    }
    println!("{}", answer);
}
