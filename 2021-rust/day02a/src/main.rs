use lazy_static::lazy_static;
use regex::Regex;
use std::{
    io::{self, Read},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl FromStr for Command {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"^(forward|down|up) (\d+)$"#).unwrap();
        }

        let caps = match RE.captures(s) {
            None => return Err(From::from(format!("{}", s))),
            Some(caps) => caps,
        };

        let val: i64 = caps[2].parse()?;
        let command_str = caps[1].to_string();

        match command_str.as_str() {
            "forward" => return Ok(Command::Forward(val)),
            "down" => return Ok(Command::Down(val)),
            "up" => return Ok(Command::Up(val)),
            _ => return Err(From::from(format!("invalid command {}", command_str))),
        }
    }
}

fn main() {
    let commands: Vec<Command> = get_input()
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect();

    let mut x = 0;
    let mut y = 0;

    for c in commands {
        match c {
            Command::Forward(val) => x += val,
            Command::Down(val) => y += val,
            Command::Up(val) => y -= val,
        };
    }

    println!("{}", x * y);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
