use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let _: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let mut meal_state: Vec<Vec<usize>> = Vec::new();
    let mut count = 0;
    let mut answer: Vec<usize> = Vec::new();
    (0..m).for_each(|_| {
        let food_info: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let _: usize = food_info[0];
        meal_state.push(food_info[1..].to_vec());
    });
    lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|b| b.parse::<usize>().unwrap())
        .for_each(|b| {
            meal_state.iter_mut().for_each(|state| {
                if state.len() == 0 {
                    return;
                }
                state.retain(|&x| x != b);
                if state.len() == 0 {
                    count += 1;
                }
            });
            answer.push(count);
        });
    answer.iter().for_each(|count| {
        println!("{}", count);
    });
}
