use std::io::{self, Read};

use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum CompassDir {
    W,
    E,
    N,
    S,
}

impl CompassDir {
    fn from_char(c: char) -> Option<CompassDir> {
        match c {
            'W' => Some(CompassDir::W),
            'E' => Some(CompassDir::E),
            'N' => Some(CompassDir::N),
            'S' => Some(CompassDir::S),
            _ => None,
        }
    }

    fn apply(&self, pos: (i64, i64), n: i64) -> (i64, i64) {
        match self {
            CompassDir::W => (pos.0 - n, pos.1),
            CompassDir::E => (pos.0 + n, pos.1),
            CompassDir::N => (pos.0, pos.1 - n),
            CompassDir::S => (pos.0, pos.1 + n),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum TurnDir {
    L,
    R,
}

impl TurnDir {
    fn from_char(c: char) -> Option<TurnDir> {
        match c {
            'L' => Some(TurnDir::L),
            'R' => Some(TurnDir::R),
            _ => None,
        }
    }

    fn apply(&self, from: CompassDir, amt: TurnAmount) -> CompassDir {
        match (from, self, amt) {
            (CompassDir::W, TurnDir::L, TurnAmount::Quarter) => CompassDir::S,
            (CompassDir::W, TurnDir::L, TurnAmount::Half) => CompassDir::E,
            (CompassDir::W, TurnDir::L, TurnAmount::ThreeQuarter) => CompassDir::N,

            (CompassDir::W, TurnDir::R, TurnAmount::Quarter) => CompassDir::N,
            (CompassDir::W, TurnDir::R, TurnAmount::Half) => CompassDir::E,
            (CompassDir::W, TurnDir::R, TurnAmount::ThreeQuarter) => CompassDir::S,

            (CompassDir::E, TurnDir::L, TurnAmount::Quarter) => CompassDir::N,
            (CompassDir::E, TurnDir::L, TurnAmount::Half) => CompassDir::W,
            (CompassDir::E, TurnDir::L, TurnAmount::ThreeQuarter) => CompassDir::S,

            (CompassDir::E, TurnDir::R, TurnAmount::Quarter) => CompassDir::S,
            (CompassDir::E, TurnDir::R, TurnAmount::Half) => CompassDir::W,
            (CompassDir::E, TurnDir::R, TurnAmount::ThreeQuarter) => CompassDir::N,

            (CompassDir::N, TurnDir::L, TurnAmount::Quarter) => CompassDir::W,
            (CompassDir::N, TurnDir::L, TurnAmount::Half) => CompassDir::S,
            (CompassDir::N, TurnDir::L, TurnAmount::ThreeQuarter) => CompassDir::E,

            (CompassDir::N, TurnDir::R, TurnAmount::Quarter) => CompassDir::E,
            (CompassDir::N, TurnDir::R, TurnAmount::Half) => CompassDir::S,
            (CompassDir::N, TurnDir::R, TurnAmount::ThreeQuarter) => CompassDir::W,

            (CompassDir::S, TurnDir::L, TurnAmount::Quarter) => CompassDir::E,
            (CompassDir::S, TurnDir::L, TurnAmount::Half) => CompassDir::N,
            (CompassDir::S, TurnDir::L, TurnAmount::ThreeQuarter) => CompassDir::W,

            (CompassDir::S, TurnDir::R, TurnAmount::Quarter) => CompassDir::W,
            (CompassDir::S, TurnDir::R, TurnAmount::Half) => CompassDir::N,
            (CompassDir::S, TurnDir::R, TurnAmount::ThreeQuarter) => CompassDir::E,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum TurnAmount {
    Quarter,
    Half,
    ThreeQuarter,
}

impl TurnAmount {
    fn from_i64(n: i64) -> Option<TurnAmount> {
        match n {
            90 => Some(TurnAmount::Quarter),
            180 => Some(TurnAmount::Half),
            270 => Some(TurnAmount::ThreeQuarter),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Order {
    Forward(i64),
    Move(CompassDir, i64),
    Turn(TurnDir, TurnAmount),
}

impl Order {
    fn from_raw(c: char, n: i64) -> Option<Order> {
        if c == 'F' {
            return Some(Order::Forward(n));
        }

        if let Some(dir) = CompassDir::from_char(c) {
            return Some(Order::Move(dir, n));
        }

        match (TurnDir::from_char(c), TurnAmount::from_i64(n)) {
            (Some(dir), Some(amt)) => Some(Order::Turn(dir, amt)),
            _ => None,
        }
    }

    fn apply(&self, s: State) -> State {
        match self {
            &Order::Forward(n) => State {
                pos: s.dir.apply(s.pos, n),
                dir: s.dir,
            },
            &Order::Move(dir, n) => State {
                pos: dir.apply(s.pos, n),
                dir: s.dir,
            },
            &Order::Turn(dir, amt) => State {
                pos: s.pos,
                dir: dir.apply(s.dir, amt),
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct State {
    pos: (i64, i64),
    dir: CompassDir,
}

fn main() {
    let re = Regex::new(r#"([NSWEFLR])(\d+)"#).unwrap();
    let orders: Vec<Order> = get_input()
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Order::from_raw(caps[1].parse().unwrap(), caps[2].parse().unwrap()).unwrap()
        })
        .collect();

    let end = orders.iter().fold(
        State {
            pos: (0, 0),
            dir: CompassDir::E,
        },
        |accum, order| order.apply(accum),
    );

    println!("{}", end.pos.0.abs() + end.pos.1.abs());
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
