use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, io, io::Read, str::FromStr};

#[derive(Debug)]
struct Content {
    color: String,
    num: usize,
}

impl FromStr for Content {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<num>\d+) (?P<color>[a-z ]+) bags?$").unwrap();
        }

        let caps = match RE.captures(s) {
            None => return Err(From::from(format!("invalid Content: {}", s))),
            Some(caps) => caps,
        };

        return Ok(Content {
            color: caps["color"].parse().unwrap(),
            num: caps["num"].parse().unwrap(),
        });
    }
}

#[derive(Debug)]
struct Rule {
    color: String,
    contents: Vec<Content>,
}

impl FromStr for Rule {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<color>[a-z ]+) bags contain (?P<contents>[a-z0-9, ]+)\.$")
                    .unwrap();
        }

        let caps = match RE.captures(s) {
            None => return Err(From::from(format!("invalid Rule: {}", s))),
            Some(caps) => caps,
        };

        let contents: Vec<Content> = caps["contents"]
            .parse::<String>()
            .unwrap()
            .split(",")
            .map(|s| s.trim())
            .filter(|&s| s != "no other bags")
            .map(|s| s.parse().unwrap())
            .collect();

        return Ok(Rule {
            color: caps["color"].parse().unwrap(),
            contents: contents,
        });
    }
}

fn count_contained(rules_by_color: &HashMap<String, Vec<Content>>, container: &str) -> usize {
    match rules_by_color.get(container) {
        None => 0,
        Some(contents) => contents.iter().fold(0, |accum, content| {
            accum + content.num * (1 + count_contained(rules_by_color, content.color.as_str()))
        }),
    }
}

fn main() {
    let input = get_input();
    let mut rules: HashMap<String, Vec<Content>> = HashMap::new();
    for rule in input.lines().map(|s| s.parse::<Rule>().unwrap()) {
        rules.insert(rule.color, rule.contents);
    }
    println!("{}", count_contained(&rules, "shiny gold"));
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
