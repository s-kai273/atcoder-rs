use std::{collections::HashMap, io::BufRead, str::SplitWhitespace};

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter: SplitWhitespace = line.split_whitespace();
    let _: u32 = iter.next().unwrap().parse().unwrap();
    let l: u32 = iter.next().unwrap().parse().unwrap();
    let mut points: HashMap<u32, Vec<u32>> = (0..l).map(|i| (i, Vec::new())).collect();
    let mut current_point = 0;
    points.get_mut(&0).unwrap().push(0);
    lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .for_each(|(i, d)| {
            let d: u32 = d.parse().unwrap();
            current_point = (current_point + d) % l;
            points.get_mut(&current_point).unwrap().push((i + 1) as u32);
        });
    let answer: usize = if l % 3 == 0 {
        let offset = l / 3;
        (0..offset)
            .map(|i| {
                points[&i].iter().count()
                    * points[&(i + offset)].iter().count()
                    * points[&(i + 2 * offset)].iter().count()
            })
            .sum()
    } else {
        0
    };
    println!("{}", answer);
}
