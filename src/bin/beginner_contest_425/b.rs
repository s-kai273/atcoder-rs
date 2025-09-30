fn main() {
    let mut lines = std::io::stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let a_list: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();
    let mut exist_list: Vec<bool> = vec![false; n];
    let mut answer: &str = "Yes";
    for i in 0..n {
        let a: i32 = a_list[i];
        if a == -1 {
            continue;
        }
        if exist_list[a as usize - 1] {
            answer = "No";
            break;
        } else {
            exist_list[a as usize - 1] = true;
        }
    }
    println!("{}", answer);
    if answer == "Yes" {
        let p_list: Vec<String> = a_list
            .iter()
            .map(|&a| {
                if a == -1 {
                    for i in 0..n {
                        if !exist_list[i] {
                            exist_list[i] = true;
                            return (i + 1).to_string();
                        }
                    }
                    unreachable!();
                } else {
                    return a.to_string();
                }
            })
            .collect();
        println!("{}", p_list.join(" "));
    }
}
