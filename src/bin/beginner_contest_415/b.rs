use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let s: String = lines.next().unwrap().unwrap();
    let ch_filtered: Vec<usize> = s
        .chars()
        .enumerate()
        .filter_map(|(i, ch)| if ch == '#' { Some(i + 1) } else { None })
        .collect();
    ch_filtered.chunks(2).for_each(|chunks| {
        println!("{},{}", chunks[0], chunks[1]);
    });
}
