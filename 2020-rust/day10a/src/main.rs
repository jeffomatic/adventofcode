use std::io::{self, Read};

fn main() {
    let mut adapters: Vec<i64> = vec![0];
    for line in get_input().lines() {
        adapters.push(line.parse().unwrap());
    }
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);

    let mut ones = 0;
    let mut threes = 0;
    for i in 0..(adapters.len() - 1) {
        let d = adapters[i + 1] - adapters[i];
        if d == 1 {
            ones += 1;
        } else if d == 3 {
            threes += 1;
        }
    }

    println!("{} * {} = {}", ones, threes, ones * threes);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
