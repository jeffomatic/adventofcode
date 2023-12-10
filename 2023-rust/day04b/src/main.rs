use std::io::{self, Read};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Card {
    want: Vec<i64>,
    got: Vec<i64>,
}

fn parse(lines: &mut dyn Iterator<Item = &str>) -> Vec<Card> {
    lines
        .map(|line| {
            let (_, nums) = line.split_once(":").unwrap();
            let (want_src, got_src) = nums.split_once("|").unwrap();
            Card {
                want: want_src
                    .trim()
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
                got: got_src
                    .trim()
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

fn matches(card: &Card) -> usize {
    card.got.iter().filter(|v| card.want.contains(v)).count()
}

fn main() {
    let input = get_input();
    let card_matches: Vec<usize> = parse(&mut input.split("\n")).iter().map(matches).collect();
    let mut copies = vec![1; card_matches.len()];
    for (i, &m) in card_matches.iter().enumerate() {
        for j in 1..=m {
            copies[i + j] += copies[i]
        }
    }
    let res: usize = copies.iter().sum();
    println!("{}", res)
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
