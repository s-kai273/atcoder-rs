fn main() {
    let mut lines = std::io::stdin().lines();
    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let query_list: Vec<String> = (0..q)
        .map(|_| lines.next().unwrap().unwrap().trim().to_string())
        .collect();
    let mut card_stack: Vec<u32> = vec![0; 100];
    query_list.iter().for_each(|query| {
        let query_parts: Vec<u32> = query
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        match query_parts[0] {
            1 => {
                card_stack.push(query_parts[1]);
            }
            2 => {
                if let Some(card) = card_stack.pop() {
                    println!("{}", card);
                }
            }
            _ => unreachable!("invalid query"),
        }
    });
}
