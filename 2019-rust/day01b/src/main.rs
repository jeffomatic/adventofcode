use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}

fn fuel_required(mass: i64) -> i64 {
    let fuel = (mass / 3) - 2;
    if fuel < 0 {
        return 0;
    }
    return fuel + fuel_required(fuel);
}

fn main() {
    println!(
        "{}",
        get_input()
            .lines()
            .map(|s| fuel_required(s.parse::<i64>().unwrap()))
            .fold(0, |acc, v| acc + v)
    );
}
