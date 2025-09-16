fn main() {
    let mut lines = std::io::stdin().lines();
    let s: String = lines.next().unwrap().unwrap().trim().to_string();
    let s_parts: Vec<u32> = s.split('-').map(|num| num.parse().unwrap()).collect();
    let mut iter = s_parts.into_iter();
    let world: u32 = iter.next().unwrap();
    let stage: u32 = iter.next().unwrap();
    let next_world = if stage < 8 { world } else { world + 1 };
    let next_stage = if stage < 8 { stage + 1 } else { 1 };
    println!("{}-{}", next_world, next_stage);
}
