use std::io::{self, Read};

fn main() {
    let input = get_input();
    let vals: Vec<i64> = input
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect();

    for i in 0..vals.len() {
        for j in i..vals.len() {
            if vals[i] + vals[j] == 2020 {
                println!("{}", vals[i] * vals[j]);
                return;
            }
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
