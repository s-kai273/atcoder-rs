use std::char;
use std::io;

fn press_buttun_a(value: &mut String, count: u32) {
    for _ in 0..count {
        value.push('0');
    }
}

fn press_button_b(value: &mut String, count: u32) {
    let value_bytes = unsafe { value.as_bytes_mut() };
    let count = count as u8;
    for b in value_bytes.iter_mut() {
        let d = *b - b'0';
        *b = (d + count) % 10 + b'0';
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let chars: Vec<char> = input.trim().chars().collect();
    let mut count = 1;
    let mut value = String::new();
    press_buttun_a(&mut value, 1);

    for pair in chars.windows(2) {
        let diff = {
            let d0 = pair[0].to_digit(10).unwrap() as i32;
            let d1 = pair[1].to_digit(10).unwrap() as i32;
            ((d0 - d1 + 10) % 10) as u32
        };
        press_button_b(&mut value, diff);
        count += diff;
        press_buttun_a(&mut value, 1);
        count += 1;
    }

    let s: String = chars.into_iter().collect();
    for _ in 0..10 {
        if value == s {
            break;
        }
        press_button_b(&mut value, 1);
        count += 1;
    }
    println!("{}", count);
}
