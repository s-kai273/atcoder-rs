use std::char;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let chars: Vec<char> = input.trim().chars().collect();
    let mut count = 1;
    let mut value = String::from("0");

    for pair in chars.windows(2) {
        let diff = {
            let d0 = pair[0].to_digit(10).unwrap() as i32;
            let d1 = pair[1].to_digit(10).unwrap() as i32;
            ((d0 - d1 + 10) % 10) as u32
        };
        count += diff;

        let mut new_value = String::new();
        for c in value.chars() {
            let digit = c.to_digit(10).unwrap();
            new_value.push(char::from_digit((digit + diff) % 10, 10).unwrap());
        }
        value = new_value;
        value.push('0');
        count += 1;
    }

    let s: String = chars.into_iter().collect();
    for _ in 0..10 {
        if value == s {
            break;
        }
        let mut new_value = String::new();
        for c in value.chars() {
            let digit = c.to_digit(10).unwrap();
            new_value.push(char::from_digit((digit + 1) % 10, 10).unwrap());
        }
        value = new_value;
        count += 1;
    }
    println!("{}", count);
}
