use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::str::FromStr;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}

#[derive(Debug)]
struct Rule {
    step: char,
    dependency: char,
}

impl FromStr for Rule {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
          /*
            Step I must be finished before step Q can begin.
            Step B must be finished before step O can begin.
          */
          static ref RE: Regex = Regex::new(r"Step (?P<dependency>[A-Z]) must be finished before step (?P<step>[A-Z]) can begin.").unwrap();
        }

        let caps = match RE.captures(s) {
            None => return Err(From::from("invalid input")),
            Some(caps) => caps,
        };

        return Ok(Rule {
            step: caps["step"].parse::<char>().unwrap(),
            dependency: caps["dependency"].parse::<char>().unwrap(),
        });
    }
}

fn remove_first(set: &mut HashSet<char>) -> char {
    let first = {
        let mut sorted: Vec<_> = set.iter().collect();
        sorted.sort();
        sorted[0].clone()
    };

    set.remove(&first);
    first
}

fn main() {
    let rules: Vec<Rule> = get_input().lines().map(|s| s.parse().unwrap()).collect();
    let mut available: HashSet<char> = (b'A'..b'Z').map(|n| n as char).collect();

    let mut deps_by_step: HashMap<char, HashSet<char>> = HashMap::new();
    for r in rules.iter() {
        // any rule means the step isn't currently available
        available.remove(&r.step);

        // add to dependency graph
        let mut deps = match deps_by_step.get(&r.step) {
            Some(deps) => deps.clone(),
            None => HashSet::new(),
        };
        deps.insert(r.dependency);
        deps_by_step.insert(r.step, deps);
    }

    while !available.is_empty() {
        let s = remove_first(&mut available);
        print!("{}", s);

        let mut next_deps_by_step = HashMap::new();
        for (step, deps) in deps_by_step.iter() {
            let next_deps: HashSet<char> = deps.iter().cloned().filter(|d| *d != s).collect();
            if next_deps.is_empty() {
                available.insert(*step);
                continue;
            }

            next_deps_by_step.insert(*step, next_deps);
        }

        deps_by_step = next_deps_by_step;
    }

    println!("");
}
