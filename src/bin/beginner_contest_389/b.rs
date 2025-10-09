fn main() {
    let mut lines = std::io::stdin().lines();
    let x: u128 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut factorial: u128 = 1;
    for i in 1..1000 {
        factorial *= i as u128;
        if factorial == x {
            println!("{}", i);
            break;
        }
    }
}
