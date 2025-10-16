fn main() {
    let mut lines = std::io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let x: String = iter.next().unwrap().trim().to_string();
    let y: String = iter.next().unwrap().trim().to_string();
    let os_list: Vec<&str> = Vec::from(["Ocelot", "Serval", "Lynx"]);
    let x_index: usize = os_list.iter().position(|&os| os == x.as_str()).unwrap();
    let y_index: usize = os_list.iter().position(|&os| os == y.as_str()).unwrap();
    let answer: &str = if x_index >= y_index { "Yes" } else { "No" };
    println!("{}", answer);
}
