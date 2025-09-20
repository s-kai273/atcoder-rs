use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let mut lines = std::io::stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut learned_list: Vec<usize> = Vec::new();
    let mut relation: HashMap<usize, HashSet<usize>> = HashMap::new();
    (0..n).for_each(|i| {
        let nums: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let a: usize = nums[0];
        let b: usize = nums[1];
        if a == 0 && b == 0 {
            learned_list.push(i + 1);
        }
        if a != 0 {
            relation.entry(a).or_insert_with(HashSet::new).insert(i + 1);
        }
        if b != 0 {
            relation.entry(b).or_insert_with(HashSet::new).insert(i + 1);
        }
    });

    let mut answer: Vec<usize> = Vec::new();
    let mut visited: Vec<usize> = vec![0; n + 1];
    let mut queue: VecDeque<usize> = VecDeque::from(learned_list);
    loop {
        if queue.len() <= 0 {
            break;
        }
        let value: usize = queue.pop_front().unwrap();
        if visited[value] == 0 {
            answer.push(value);
            visited[value] = 1;
            if let Some(rel) = relation.get(&value) {
                queue.extend(rel);
            }
        }
    }
    println!("{}", answer.len());
}
