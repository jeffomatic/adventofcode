use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::io::{self, Read};

fn get_input() -> String {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();
    handle.read_to_string(&mut input).unwrap();
    return input;
}

struct Claim {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

lazy_static! {
    // #1 @ 167,777: 23x12
    static ref CLAIM_REGEX: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
}

fn deserialize_claim(s: &str) -> Option<Claim> {
    let caps = CLAIM_REGEX.captures(s)?;
    return Some(Claim {
        x: caps.get(2)?.as_str().parse().ok()?,
        y: caps.get(3)?.as_str().parse().ok()?,
        w: caps.get(4)?.as_str().parse().ok()?,
        h: caps.get(5)?.as_str().parse().ok()?,
    });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let claims: Vec<_> = get_input()
        .lines()
        .map(|s| deserialize_claim(s).unwrap())
        .collect();

    let mut claimed = HashSet::new();
    let mut overclaimed = HashSet::new();

    for c in claims {
        for i in 0..c.h {
            for j in 0..c.w {
                let p = (c.y + i, c.x + j);
                if claimed.contains(&p) {
                    overclaimed.insert(p);
                    continue;
                }

                claimed.insert(p);
            }
        }
    }

    println!("{}", overclaimed.len());
    Ok(())
}
