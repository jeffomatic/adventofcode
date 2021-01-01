use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::io::{self, Read};
use std::str::FromStr;

fn get_input() -> String {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();
    handle.read_to_string(&mut input).unwrap();
    return input;
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Claim {
    id: i32,
    p: Point,
    w: i32,
    h: i32,
}

impl Claim {
    fn corners(&self) -> (Point, Point) {
        (
            self.p,
            Point {
                x: self.p.x + self.w - 1,
                y: self.p.y + self.h - 1,
            },
        )
    }

    fn overlaps(&self, other: &Self) -> bool {
        let (a1, a2) = self.corners();
        let (b1, b2) = other.corners();
        ((b1.x <= a1.x && a1.x <= b2.x) || (a1.x <= b1.x && b1.x <= a2.x))
            && ((b1.y <= a1.y && a1.y <= b2.y) || (a1.y <= b1.y && b1.y <= a2.y))
    }

    fn _from_str(s: &str) -> Option<Claim> {
        lazy_static! {
            // #1 @ 167,777: 23x12
            static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }

        let caps = RE.captures(s)?;
        return Some(Claim {
            id: caps.get(1)?.as_str().parse().ok()?,
            p: Point {
                x: caps.get(2)?.as_str().parse().ok()?,
                y: caps.get(3)?.as_str().parse().ok()?,
            },
            w: caps.get(4)?.as_str().parse().ok()?,
            h: caps.get(5)?.as_str().parse().ok()?,
        });
    }
}

impl FromStr for Claim {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Claim, Self::Err> {
        match Self::_from_str(s) {
            Some(c) => Ok(c),
            None => Err(From::from("couldn't parse claim")),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let claims: Vec<Claim> = get_input().lines().map(|s| s.parse().unwrap()).collect();

    let mut overlappers = HashSet::new();

    for i in 0..claims.len() {
        let a = &claims[i];
        for j in (i + 1)..claims.len() {
            let b = &claims[j];
            if a.overlaps(&b) {
                overlappers.insert(&a.id);
                overlappers.insert(&b.id);
            }
        }

        if !overlappers.contains(&a.id) {
            println!("{}", a.id);
            return Ok(());
        }
    }

    Err(From::from("none found"))
}
