use std::io::{self, Read};

fn main() {
    let input = get_input();
    let res: u32 = input
        .split("\n")
        .map(|line| {
            let digits = line
                .chars()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            10 * digits.first().unwrap() + digits.last().unwrap()
        })
        .sum();
    println!("{}", res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
