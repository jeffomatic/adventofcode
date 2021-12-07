use std::io::{self, Read};

fn main() {
    let positions: Vec<i32> = get_input()
        .split(',')
        .map(|str| str.parse().unwrap())
        .collect();

    let (min, max) = positions
        .iter()
        .fold((i32::MAX, i32::MIN), |(min, max), pos| {
            (*pos.min(&min), *pos.max(&max))
        });

    let mut best_cost = i32::MAX;
    for i in min..=max {
        let cost = positions.iter().fold(0, |accum, pos| {
            let n = (pos - i).abs();
            let c = n * (n + 1) / 2;
            accum + c
        });
        best_cost = best_cost.min(cost);
    }

    println!("{}", best_cost);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
