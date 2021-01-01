use std::io::{self, Read};

fn main() {
    let input: Vec<String> = get_input().lines().map(|s| s.to_string()).collect();
    let buses: Vec<(i64, i64)> = input[1]
        .split(",")
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(n, s)| {
            let period: i64 = s.parse().unwrap();
            let offset: i64 = n as i64;
            (period, offset)
        })
        .collect();

    let (mut period, mut t) = buses[0];
    for (other_period, other_offset) in buses.iter().skip(1) {
        while (t + other_offset) % other_period != 0 {
            t += period;
        }
        period *= other_period;
    }

    println!("{}", t);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
