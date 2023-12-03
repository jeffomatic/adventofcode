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
    draws: Vec<Vec<Draw>>,
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
    let (_, post) = s.split_once(":").unwrap();
    Game {
        draws: post
            .split(";")
            .map(|s| s.split(",").map(parse_draw).collect())
            .collect(),
    }
}

fn min_count(drawsets: &Vec<Vec<Draw>>) -> (i32, i32, i32) {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    for draws in drawsets {
        for d in draws {
            match d.c {
                Color::Red => r = i32::max(r, d.n),
                Color::Green => g = i32::max(g, d.n),
                Color::Blue => b = i32::max(b, d.n),
            }
        }
    }

    (r, g, b)
}

fn main() {
    let input = get_input();
    let games: Vec<Game> = input.split("\n").map(parse_game).collect();
    let res: i32 = games
        .into_iter()
        .map(|g| min_count(&g.draws))
        .map(|(r, g, b)| r * g * b)
        .sum();
    println!("{:?}", res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
