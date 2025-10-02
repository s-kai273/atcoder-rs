fn main() {
    let mut lines = std::io::stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut answer: Vec<Vec<char>> = vec![vec!['?'; n]; n];
    for i in 0..n {
        let j: usize = n - (i + 1);
        if i <= j {
            let color: char = if i % 2 == 0 { '#' } else { '.' };
            for x in i..j + 1 {
                answer[i][x] = color;
                answer[j][x] = color;
                answer[x][i] = color;
                answer[x][j] = color;
            }
        }
    }
    answer.iter().for_each(|row| {
        println!("{}", row.iter().collect::<String>());
    });
}
