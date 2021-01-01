use std::io::{self, Read};

fn main() {
    let input: Vec<String> = get_input().lines().map(|s| s.to_string()).collect();
    let depart_at: i64 = input[0].parse().unwrap();
    let buses: Vec<i64> = input[1].split(",").fold(Vec::new(), |mut accum, s| {
        if s == "x" {
            accum
        } else {
            accum.push(s.parse().unwrap());
            accum
        }
    });

    let mut best_bus = -1;
    let mut best_time = i64::MAX;
    for &b in &buses {
        let rem = depart_at % b;
        let t = if rem == 0 { 0 } else { b - depart_at % b };

        if t < best_time {
            best_time = t;
            best_bus = b;
        }
    }

    println!("{}", best_time * best_bus)
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
