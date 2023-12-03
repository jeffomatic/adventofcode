use std::io::{self, Read};

fn parse(s: &str) -> Vec<u32> {
    let mut res = Vec::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, c) in chars.iter().enumerate() {
        if let Some(d) = c.to_digit(10) {
            res.push(d);
            continue;
        }

        let prefix: String = chars[0..=i].into_iter().collect();
        if prefix.ends_with("one") {
            res.push(1);
            continue;
        }
        if prefix.ends_with("two") {
            res.push(2);
            continue;
        }
        if prefix.ends_with("three") {
            res.push(3);
            continue;
        }
        if prefix.ends_with("four") {
            res.push(4);
            continue;
        }
        if prefix.ends_with("five") {
            res.push(5);
            continue;
        }
        if prefix.ends_with("six") {
            res.push(6);
            continue;
        }
        if prefix.ends_with("seven") {
            res.push(7);
            continue;
        }
        if prefix.ends_with("eight") {
            res.push(8);
            continue;
        }
        if prefix.ends_with("nine") {
            res.push(9);
            continue;
        }
    }
    res
}

fn main() {
    let input = get_input();
    let res: u32 = input
        .split("\n")
        .map(|line| {
            let digits = parse(line);
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
