use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut c_list: Vec<String> = Vec::new();
    let mut l_list: Vec<usize> = Vec::new();
    (0..n).for_each(|_| {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let c: String = iter.next().unwrap().to_string();
        let l: usize = iter.next().unwrap().parse().unwrap();
        c_list.push(c);
        l_list.push(l);
    });

    let mut s = String::new();
    let mut is_too_long = false;
    for i in 0..n {
        let c = c_list.get(i).unwrap();
        let l = l_list.get(i).unwrap();
        if s.len() + l > 100 {
            is_too_long = true;
            break;
        }
        s += c.as_str().repeat(*l).as_str();
    }
    if is_too_long {
        println!("Too Long");
    } else {
        println!("{}", s);
    }
}
