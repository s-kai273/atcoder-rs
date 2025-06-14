use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut answer_list: Vec<String> = Vec::new();
    (0..t).for_each(|_| {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let line = lines.next().unwrap().unwrap();
        let mut c_list: Vec<char> = line.chars().collect();
        let mut from_index = 0;
        for i in 0..n - 1 {
            from_index = i;
            if c_list[i] > c_list[i + 1] {
                break;
            }
        }
        let mut to_index = from_index;
        for i in from_index + 1..n {
            if c_list[i] > c_list[from_index] {
                break;
            }
            to_index = i + 1;
        }
        let ch = c_list[from_index];
        c_list.insert(to_index, ch);
        c_list.remove(from_index);
        let answer: String = c_list.iter().collect();
        answer_list.push(answer);
    });
    answer_list.iter().for_each(|answer| println!("{}", answer));
}
