use regex::Regex;
use std::io::{self, Read};

fn main() {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    let input = get_input();
    let mut valid = 0;

    for s in input.split("\n") {
        let caps = re.captures(s).unwrap();
        let p1 = caps[1].parse::<usize>().unwrap();
        let p2 = caps[2].parse::<usize>().unwrap();
        let want = caps[3].parse::<char>().unwrap();
        let pw = caps[4].chars().collect::<Vec<char>>();

        let mut got = 0;
        if pw[p1 - 1] == want {
            got += 1
        }
        if pw[p2 - 1] == want {
            got += 1
        }

        if got == 1 {
            valid += 1
        }
    }

    println!("{}", valid);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
