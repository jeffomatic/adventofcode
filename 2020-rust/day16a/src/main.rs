use std::{
    collections::HashMap,
    io::{self, Read},
    ops::RangeInclusive,
};

use regex::Regex;

fn main() {
    let chunks: Vec<String> = get_input().split("\n\n").map(|s| s.to_string()).collect();

    let mut rules: HashMap<String, Vec<RangeInclusive<i64>>> = HashMap::new();
    let rule_re = Regex::new(r#"([^:]+): (\d+)-(\d+) or (\d+)-(\d+)"#).unwrap();
    for line in chunks[0].lines() {
        let caps = rule_re.captures(line).unwrap();
        let k = caps[1].to_string();
        let a1: i64 = caps[2].parse().unwrap();
        let a2: i64 = caps[3].parse().unwrap();
        let b1: i64 = caps[4].parse().unwrap();
        let b2: i64 = caps[5].parse().unwrap();
        rules.insert(k, vec![(a1..=a2), (b1..=b2)]);
    }

    // let my_ticket: Vec<i64> = chunks[1]
    //     .lines()
    //     .last()
    //     .unwrap()
    //     .split(",")
    //     .map(|num| num.parse().unwrap())
    //     .collect();

    let other_tickets: Vec<Vec<i64>> = chunks[2]
        .lines()
        .skip(1)
        .map(|line| line.split(",").map(|num| num.parse().unwrap()).collect())
        .collect();

    let err_rate = other_tickets.iter().fold(0, |accum, ticket| {
        ticket.iter().fold(accum, |accum, num| {
            for r in rules.values().flatten() {
                if r.contains(num) {
                    return accum;
                }
            }
            accum + num
        })
    });

    println!("{}", err_rate);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
