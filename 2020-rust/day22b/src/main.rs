use std::{
    collections::{HashSet, VecDeque},
    io::{self, Read},
};

type GameResult = (bool, VecDeque<usize>);
type Deck = VecDeque<usize>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Game {
    deck_a: Deck,
    deck_b: Deck,
}

// Returns true if deck_a is the winner.
fn play(mut game: Game) -> GameResult {
    let mut history: HashSet<Game> = HashSet::new();

    while !game.deck_a.is_empty() && !game.deck_b.is_empty() {
        if history.contains(&game) {
            let res = (true, game.deck_a);
            // cache.insert(snapshot, res.clone());
            return res;
        }
        history.insert(game.clone());

        let a = game.deck_a.pop_front().unwrap();
        let b = game.deck_b.pop_front().unwrap();

        let a_wins = if game.deck_a.len() >= a && game.deck_b.len() >= b {
            play(Game {
                deck_a: game.deck_a.iter().take(a).cloned().collect(),
                deck_b: game.deck_b.iter().take(b).cloned().collect(),
            })
            .0
        } else {
            a > b
        };

        if a_wins {
            game.deck_a.push_back(a);
            game.deck_a.push_back(b);
        } else {
            game.deck_b.push_back(b);
            game.deck_b.push_back(a);
        }
    }

    if game.deck_b.is_empty() {
        (true, game.deck_a)
    } else {
        (false, game.deck_b)
    }
}

fn main() {
    let decks: Vec<VecDeque<usize>> = get_input()
        .split("\n\n")
        .map(|chunk| chunk.lines().skip(1).map(|s| s.parse().unwrap()).collect())
        .collect();

    let (_, winning_deck) = play(Game {
        deck_a: decks[0].clone(),
        deck_b: decks[1].clone(),
    });

    let score = winning_deck
        .iter()
        .enumerate()
        .fold(0, |accum, (i, v)| accum + v * (winning_deck.len() - i));
    println!("{}", score);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
