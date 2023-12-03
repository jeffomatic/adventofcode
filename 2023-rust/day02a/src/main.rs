use std::io::{self, Read};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Draw {
    c: Color,
    n: i32,
}

#[derive(Debug)]
struct Game {
    n: i32,
    draws: Vec<Vec<Draw>>,
}

fn parse_game_number(s: &str) -> i32 {
    s.trim().split_once(" ").unwrap().1.trim().parse().unwrap()
}

fn parse_draw(s: &str) -> Draw {
    let (pre, post) = s.trim().split_once(" ").unwrap();
    Draw {
        n: pre.trim().parse().unwrap(),
        c: match post.trim() {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("invalid color {}", post),
        },
    }
}

fn parse_game(s: &str) -> Game {
    let (pre, post) = s.split_once(":").unwrap();
    Game {
        n: parse_game_number(pre),
        draws: post
            .split(";")
            .map(|s| s.split(",").map(parse_draw).collect())
            .collect(),
    }
}

fn max(c: Color) -> i32 {
    match c {
        Color::Red => 12,
        Color::Green => 13,
        Color::Blue => 14,
    }
}

fn valid_game(g: &Game) -> bool {
    for drawset in g.draws.iter() {
        for d in drawset.iter() {
            if d.n > max(d.c) {
                return false;
            }
        }
    }
    true
}

fn main() {
    let input = get_input();
    let games: Vec<Game> = input.split("\n").map(parse_game).collect();
    let res: i32 = games.into_iter().filter(valid_game).map(|g| g.n).sum();
    println!("{:?}", res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
