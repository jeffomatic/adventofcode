use std::io::{self, Read};

// 0: 6 signals
// 1: 2 signals
// 2: 5 signals
// 3: 5 signals
// 4: 4 signals
// 5: 5 signals
// 6: 6 signals
// 7: 3 signals
// 8: 7 signals
// 9: 6 signals

fn main() {
    let res = get_input()
        .lines()
        .map(|line| {
            line.split(" | ")
                .skip(1)
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .filter(|tok| match tok.len() {
                    // actual digits: 1 | 4 | 7 | 8
                    2 | 4 | 3 | 7 => true,
                    _ => false,
                })
                .count() as i64
        })
        .sum::<i64>();
    dbg!(res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
