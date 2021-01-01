use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Command {
    Cut(i64),
    DealNew,
    DealWithIncrement(usize),
}

impl FromStr for Command {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<command>[A-Za-z ]+)(?P<arg>-?\d+)?").unwrap();
        }

        let caps = match RE.captures(s) {
            None => return Err(From::from("invalid command format")),
            Some(caps) => caps,
        };

        match caps["command"].parse::<String>().unwrap().as_str() {
            "cut " => Ok(Command::Cut(caps["arg"].parse::<i64>().unwrap())),
            "deal into new stack" => Ok(Command::DealNew),
            "deal with increment " => Ok(Command::DealWithIncrement(
                caps["arg"].parse::<usize>().unwrap(),
            )),
            _ => return Err(From::from("unrecognized command")),
        }
    }
}

#[derive(Debug)]
struct Deck {
    q: VecDeque<usize>,
}

impl Deck {
    fn new(size: usize) -> Deck {
        let mut q = VecDeque::new();
        for n in 0..size {
            q.push_back(n);
        }
        Deck { q: q }
    }

    fn cut(&mut self, amount: i64) {
        if amount > 0 {
            for _ in 0..amount {
                let n = self.q.pop_front().unwrap();
                self.q.push_back(n);
            }
        } else {
            for _ in 0..amount.abs() {
                let n = self.q.pop_back().unwrap();
                self.q.push_front(n);
            }
        }
    }

    // this is equivalent to revsering the deck
    fn deal_new(&mut self) {
        for i in 0..(self.q.len() / 2) {
            self.q.swap(i, self.q.len() - 1 - i);
        }
    }

    fn deal_with_increment(&mut self, increment: usize) {
        let mut v = Vec::new();
        for _ in 0..self.q.len() {
            v.push(usize::max_value()); // use a sentinel value here
        }

        let mut index = 0;
        while let Some(n) = self.q.pop_front() {
            v[index] = n;
            index = (index + increment) % v.len();
        }

        for n in v.iter() {
            self.q.push_back(*n);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let commands: Vec<Command> = input
        .trim()
        .to_string()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut deck = Deck::new(10007);
    for c in commands {
        match c {
            Command::Cut(n) => deck.cut(n),
            Command::DealNew => deck.deal_new(),
            Command::DealWithIncrement(n) => deck.deal_with_increment(n),
        }
    }

    for (i, v) in deck.q.iter().enumerate() {
        if *v == 2019 {
            println!("{}", i);
            break;
        }
    }
}
