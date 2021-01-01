use std::{
    collections::VecDeque,
    io::{self, Read},
};

fn main() {
    let decks: Vec<VecDeque<i64>> = get_input()
        .split("\n\n")
        .map(|chunk| chunk.lines().skip(1).map(|s| s.parse().unwrap()).collect())
        .collect();
    let (mut deck_a, mut deck_b) = (decks[0].clone(), decks[1].clone());

    while !deck_a.is_empty() && !deck_b.is_empty() {
        let a = deck_a.pop_front().unwrap();
        let b = deck_b.pop_front().unwrap();

        if a > b {
            deck_a.push_back(a);
            deck_a.push_back(b);
        } else {
            deck_b.push_back(b);
            deck_b.push_back(a);
        }
    }

    let winner = if deck_a.is_empty() { &deck_b } else { &deck_a };
    let score = winner.iter().enumerate().fold(0, |accum, (i, v)| {
        accum + v * (winner.len() as i64 - i as i64)
    });
    println!("{}", score);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
