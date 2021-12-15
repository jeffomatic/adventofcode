use std::{
    collections::HashMap,
    io::{self, Read},
};

fn main() {
    let input = get_input();
    let mut lines = input.lines();

    // grab template
    let state: Vec<char> = lines.next().unwrap().chars().collect();

    // consume a blank line
    lines.next();

    // parse rules
    let mut rules: HashMap<(char, char), char> = HashMap::new();
    let mut pairs: Vec<(char, char)> = Vec::new();
    let mut pair_counts: HashMap<(char, char), usize> = HashMap::new();
    for line in lines {
        let in_a = line.chars().nth(0).unwrap();
        let in_b = line.chars().nth(1).unwrap();
        let out = line.chars().nth(6).unwrap();
        rules.insert((in_a, in_b), out);
        pairs.push((in_a, in_b));
        pair_counts.insert((in_a, in_b), 0);
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in state.iter() {
        counts.entry(*c).and_modify(|val| *val += 1).or_insert(1);
    }

    for i in 0..(state.len() - 1) {
        pair_counts
            .entry((state[i], state[i + 1]))
            .and_modify(|val| *val += 1);
    }

    for _ in 0..10 {
        let mut next_pair_counts = pair_counts.clone();

        for (in_a, in_b) in pairs.iter() {
            let pair_count = *pair_counts.get(&(*in_a, *in_b)).unwrap();
            if pair_count < 1 {
                continue;
            }

            let new_element = *rules.get(&(*in_a, *in_b)).unwrap();
            counts
                .entry(new_element)
                .and_modify(|val| *val += pair_count)
                .or_insert(pair_count);

            next_pair_counts
                .entry((*in_a, *in_b))
                .and_modify(|val| *val -= pair_count);
            next_pair_counts
                .entry((*in_a, new_element))
                .and_modify(|val| *val += pair_count);
            next_pair_counts
                .entry((new_element, *in_b))
                .and_modify(|val| *val += pair_count);
        }

        pair_counts = next_pair_counts;
    }

    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();

    println!("{}", max - min);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
