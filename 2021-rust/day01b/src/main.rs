use std::io::{self, Read};

fn main() {
    let input = get_input();
    let vals: Vec<i64> = input
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect();

    let mut res = 0;
    for i in 0..(vals.len() - 3) {
        if vals[i] + vals[i+1] + vals[i+2] < vals[i+1] + vals[i+2] + vals[i+3] {
            res += 1;
        }
    }

    println!("{}", res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
