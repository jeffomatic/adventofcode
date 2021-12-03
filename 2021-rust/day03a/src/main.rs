use std::io::{self, Read};

fn main() {
    let input: Vec<Vec<char>> = get_input()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
    let num_digits = input[0].len();

    let mut digit_counts = Vec::new();
    for pos in 0..num_digits {
        let mut count = 0;

        for row in input.iter() {
            if row[pos] == '0' {
                count -= 1;
            } else {
                count += 1;
            }
        }

        digit_counts.push(count);
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for (pos, count) in digit_counts.iter().enumerate() {
        let exp = num_digits as i32 - pos as i32 - 2;
        let mul = match exp {
            -1 => 1,
            _ => 2 << exp,
        };

        if *count > 0 {
            gamma += mul;
        } else {
            epsilon += mul;
        }
    }

    println!("g: {} e: {} prod: {}", gamma, epsilon, gamma * epsilon);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
