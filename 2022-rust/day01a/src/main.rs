use std::io::{self, Read};

fn main() {
    let input = get_input();
    let res: i64 = input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split("\n")
                .map(|line| line.parse::<i64>().unwrap())
                .sum()
        })
        .max()
        .unwrap();
    println!("{}", res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
