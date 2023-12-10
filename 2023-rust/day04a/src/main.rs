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

fn score(card: &Card) -> usize {
    let matches = card.got.iter().filter(|v| card.want.contains(v)).count();
    if matches < 1 {
        return 0;
    }
    1 << (matches - 1)
}

fn main() {
    let input = get_input();
    let cards = parse(&mut input.split("\n"));
    let res = cards.iter().fold(0, |acc, c| acc + score(c));
    println!("{}", res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
