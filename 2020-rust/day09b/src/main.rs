use std::io::{self, Read};

fn main() {
    let target: i64 = 466456641;
    let vals: Vec<i64> = get_input()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    'next_start: for i in 0..vals.len() {
        let mut total: i64 = vals[i];
        for j in (i + 1)..vals.len() {
            total += vals[j];

            if total == target {
                let span = &vals[i..=j];
                println!(
                    "{}",
                    span.iter().min().unwrap() + span.iter().max().unwrap()
                );
                return;
            }

            if total > target {
                continue 'next_start;
            }
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
