fn dfs(value: u64, max_value: u64, has_three: bool, has_five: bool, has_seven: bool) -> usize {
    if value > max_value {
        return 0;
    }
    let count: usize = dfs(10 * value + 3, max_value, true, has_five, has_seven)
        + dfs(10 * value + 5, max_value, has_three, true, has_seven)
        + dfs(10 * value + 7, max_value, has_three, has_five, true);
    if has_three && has_five && has_seven {
        return count + 1;
    } else {
        return count;
    }
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let n: u64 = lines.next().unwrap().unwrap().parse().unwrap();
    let answer: usize = dfs(0, n, false, false, false);
    println!("{}", answer);
}
