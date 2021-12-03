use std::io::{self, Read};

fn binary_to_decimal(digits: &Vec<char>) -> i64 {
    let mut res = 0;
    let num_digits = digits.len();

    for (pos, c) in digits.iter().enumerate() {
        let exp = num_digits as i32 - pos as i32 - 2;
        let mul = match exp {
            -1 => 1,
            _ => 2 << exp,
        };

        if *c == '1' {
            res += mul;
        }
    }

    return res;
}

fn get_digit_bias(values: &Vec<Vec<char>>, pos: usize) -> i64 {
    values.iter().fold(0, |accum, row| {
        if row[pos] == '0' {
            accum - 1
        } else {
            accum + 1
        }
    })
}

fn main() {
    let values: Vec<Vec<char>> = get_input()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
    let num_digits = values[0].len();

    let mut oxy_vals = values.clone();
    let mut co2_vals = values.clone();

    for pos in 0..num_digits {
        if oxy_vals.len() > 1 {
            let bias = get_digit_bias(&oxy_vals, pos);
            let want_char = if bias >= 0 { '1' } else { '0' };
            oxy_vals = oxy_vals
                .iter()
                .filter(|val| val[pos] == want_char)
                .map(|v| v.clone())
                .collect();
        }

        if co2_vals.len() > 1 {
            let bias = get_digit_bias(&co2_vals, pos);
            let want_char = if bias >= 0 { '0' } else { '1' };
            co2_vals = co2_vals
                .iter()
                .filter(|val| val[pos] == want_char)
                .map(|v| v.clone())
                .collect();
        }
    }

    let oxy = binary_to_decimal(&oxy_vals[0]);
    let co2 = binary_to_decimal(&co2_vals[0]);

    println!(
        "oxy_vals: {:?} co2_vals: {:?} oxy: {} co2 {} prod: {}",
        oxy_vals,
        co2_vals,
        oxy,
        co2,
        oxy * co2
    );
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
