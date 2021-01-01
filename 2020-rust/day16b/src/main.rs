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
        let label = caps[1].to_string();
        let a1: i64 = caps[2].parse().unwrap();
        let a2: i64 = caps[3].parse().unwrap();
        let b1: i64 = caps[4].parse().unwrap();
        let b2: i64 = caps[5].parse().unwrap();
        rules.insert(label, vec![(a1..=a2), (b1..=b2)]);
    }

    let my_ticket: Vec<i64> = chunks[1]
        .lines()
        .last()
        .unwrap()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();

    let field_count = my_ticket.len();

    let other_tickets: Vec<Vec<i64>> = chunks[2]
        .lines()
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let valid_tickets: Vec<&Vec<i64>> = other_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|n| rules.values().flatten().any(|r| r.contains(n)))
        })
        .collect();

    let mut fields_for_rule: HashMap<String, Vec<usize>> = HashMap::new();
    for (label, ranges) in rules.iter() {
        let candidate_fields: Vec<usize> = (0..field_count)
            .filter(|f| {
                valid_tickets
                    .iter()
                    .map(|t| t[*f])
                    .all(|v| ranges.iter().any(|r| r.contains(&v)))
            })
            .collect();
        fields_for_rule.insert(label.clone(), candidate_fields);
    }

    let mut q: Vec<(String, usize)> = fields_for_rule
        .iter()
        .filter(|(_label, fields)| fields.len() == 1)
        .map(|(label, fields)| (label.clone(), *fields.first().unwrap()))
        .collect();

    while let Some((finished_label, claimed_field)) = q.pop() {
        for (other_label, other_fields) in fields_for_rule.iter_mut() {
            if *other_label == finished_label {
                continue;
            }

            if let Some(i) = other_fields
                .iter()
                .position(|other_field| *other_field == claimed_field)
            {
                other_fields.remove(i);
                if other_fields.len() == 1 {
                    q.push((other_label.clone(), *other_fields.first().unwrap()));
                }
            }
        }
    }

    let res = fields_for_rule
        .iter()
        .filter(|(label, _fields)| label.starts_with("departure"))
        .map(|(_label, fields)| fields[0])
        .fold(1, |accum, f| accum * my_ticket[f]);

    println!("{}", res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
