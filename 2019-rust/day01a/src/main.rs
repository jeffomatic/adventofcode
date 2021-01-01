use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}

fn main() {
    println!(
        "{}",
        get_input()
            .lines()
            .map(|s| (s.parse::<i64>().unwrap() / 3) - 2)
            .fold(0, |acc, v| acc + v)
    );
}
