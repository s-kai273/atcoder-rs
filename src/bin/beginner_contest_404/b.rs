use std::{io, vec};

fn lotate(char_list: &Vec<Vec<char>>, count: u32) -> Vec<Vec<char>> {
    let mut rotated_list: Vec<Vec<char>> = vec![vec![' '; char_list[0].len()]; char_list.len()];
    match count % 4 {
        0 => char_list.clone(),
        1 => {
            for i in 0..char_list.len() {
                for j in 0..char_list[0].len() {
                    rotated_list[j][char_list.len() - 1 - i] = char_list[i][j];
                }
            }
            rotated_list
        }
        2 => {
            for i in 0..char_list.len() {
                for j in 0..char_list[0].len() {
                    rotated_list[char_list.len() - 1 - i][char_list[0].len() - 1 - j] =
                        char_list[i][j];
                }
            }
            rotated_list
        }
        3 => {
            for i in 0..char_list.len() {
                for j in 0..char_list[0].len() {
                    rotated_list[char_list[0].len() - 1 - j][i] = char_list[i][j];
                }
            }
            rotated_list
        }
        _ => unreachable!("Modulo 4 should only yield 0, 1, 2, or 3"),
    }
}

fn calc_diff(s_list: &Vec<Vec<char>>, t_list: &Vec<Vec<char>>) -> u32 {
    let mut diff: u32 = 0;
    for i in 0..s_list.len() {
        for j in 0..s_list[0].len() {
            if s_list[i][j] != t_list[i][j] {
                diff += 1;
            }
        }
    }
    diff
}

fn main() {
    let mut lines = io::stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut s_list: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        s_list.push(lines.next().unwrap().unwrap().chars().collect());
    }
    let mut t_list: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        t_list.push(lines.next().unwrap().unwrap().chars().collect());
    }
    let mut answer = calc_diff(&s_list, &t_list);
    for i in 1..4 {
        let rotated_s_list = lotate(&s_list, i);
        let diff = calc_diff(&rotated_s_list, &t_list) + i;
        if diff < answer {
            answer = diff;
        }
    }
    println!("{}", answer);
}
