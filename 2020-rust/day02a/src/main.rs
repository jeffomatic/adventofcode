use regex::Regex;
use std::io::{self, Read};

fn main() {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    let input = get_input();
    let mut valid = 0;

    for s in input.split("\n") {
        let caps = re.captures(s).unwrap();
        let from = caps[1].parse::<i32>().unwrap();
        let to = caps[2].parse::<i32>().unwrap();
        let want = caps[3].parse::<char>().unwrap();

        let mut got = 0;
        for c in caps[4].chars() {
            if c == want {
                got += 1;
            }
        }

        if from <= got && got <= to {
            valid += 1;
        }
    }

    println!("{}", valid);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
