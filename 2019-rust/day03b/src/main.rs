use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::io::{self, Read};
use std::str::FromStr;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}

#[derive(Debug)]
struct Segment {
    dir: char,
    length: i64,
}

impl FromStr for Segment {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<dir>[UDLR])(?P<length>\d+)").unwrap();
        }

        let caps = match RE.captures(s) {
            None => return Err(From::from("invalid input")),
            Some(caps) => caps,
        };

        return Ok(Segment {
            dir: caps["dir"].parse::<char>().unwrap(),
            length: caps["length"].parse::<i64>().unwrap(),
        });
    }
}

fn points(segs: &Vec<Segment>) -> HashSet<(i64, i64)> {
    let mut res: HashSet<(i64, i64)> = HashSet::new();
    let mut pos = (0, 0);
    for s in segs {
        for _ in 1..=s.length {
            match s.dir {
                'U' => pos = (pos.0, pos.1 + 1),
                'D' => pos = (pos.0, pos.1 - 1),
                'L' => pos = (pos.0 - 1, pos.1),
                'R' => pos = (pos.0 + 1, pos.1),
                other => panic!("invalid dir {}", other),
            }
            res.insert(pos);
        }
    }
    return res;
}

// TODO: a generator would let us DRY up this function and points()
fn steps_to(segs: &Vec<Segment>, dst: &(i64, i64)) -> Option<i64> {
    let mut cur = (0, 0);
    let mut steps = 0;
    for s in segs {
        for _ in 1..=s.length {
            match s.dir {
                'U' => cur = (cur.0, cur.1 + 1),
                'D' => cur = (cur.0, cur.1 - 1),
                'L' => cur = (cur.0 - 1, cur.1),
                'R' => cur = (cur.0 + 1, cur.1),
                other => panic!("invalid dir {}", other),
            }

            steps += 1;
            if cur == *dst {
                return Some(steps);
            }
        }
    }
    return None;
}

fn main() {
    let input = get_input();
    let lines: Vec<Vec<Segment>> = input
        .lines()
        .map(|line| line.split(",").map(|s| s.parse().unwrap()).collect())
        .collect();
    let a = points(&lines[0]);
    let b = points(&lines[1]);
    let crossings = a.intersection(&b);
    let best = crossings.fold(1000000, |best, c| {
        cmp::min(
            best,
            steps_to(&lines[0], c).unwrap() + steps_to(&lines[1], c).unwrap(),
        )
    });
    println!("{}", best)
}
