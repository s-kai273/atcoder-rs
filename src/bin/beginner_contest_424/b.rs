fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let event_list: Vec<Vec<usize>> = (0..k)
        .map(|_| {
            let event: Vec<usize> = lines
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            event
        })
        .collect();
    let mut clear_count_list: Vec<usize> = vec![0; n];
    let mut answer: Vec<usize> = Vec::new();
    event_list.iter().for_each(|event| {
        let a: usize = event[0];
        let _: usize = event[1];
        clear_count_list[a - 1] += 1;
        if clear_count_list[a - 1] >= m {
            answer.push(a);
        }
    });
    println!(
        "{}",
        answer
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
