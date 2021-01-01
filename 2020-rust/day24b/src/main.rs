use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

type Pos = (i64, i64);

#[derive(Debug, Clone, Copy)]
enum Dir {
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

impl Dir {
    fn go(&self, p: &Pos) -> Pos {
        match self {
            Self::E => (p.0 + 1, p.1),
            Self::W => (p.0 - 1, p.1),
            Self::NE => {
                if p.1 % 2 == 0 {
                    (p.0, p.1 - 1)
                } else {
                    (p.0 + 1, p.1 - 1)
                }
            }
            Self::NW => {
                if p.1 % 2 == 0 {
                    (p.0 - 1, p.1 - 1)
                } else {
                    (p.0, p.1 - 1)
                }
            }
            Self::SE => {
                if p.1 % 2 == 0 {
                    (p.0, p.1 + 1)
                } else {
                    (p.0 + 1, p.1 + 1)
                }
            }
            Self::SW => {
                if p.1 % 2 == 0 {
                    (p.0 - 1, p.1 + 1)
                } else {
                    (p.0, p.1 + 1)
                }
            }
        }
    }
}

fn adjacent(p: &Pos) -> Vec<Pos> {
    vec![
        Dir::E.go(p),
        Dir::W.go(p),
        Dir::NE.go(p),
        Dir::NW.go(p),
        Dir::SE.go(p),
        Dir::SW.go(p),
    ]
}

fn main() {
    let paths: Vec<Vec<Dir>> = get_input()
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let mut dirs = Vec::new();

            let mut i = 0;
            while i < chars.len() {
                match chars[i] {
                    'e' => dirs.push(Dir::E),
                    'w' => dirs.push(Dir::W),
                    'n' => {
                        match chars[i + 1] {
                            'e' => dirs.push(Dir::NE),
                            'w' => dirs.push(Dir::NW),
                            _ => panic!("invalid char sequence {}, {}", chars[i], chars[i + 1]),
                        }
                        i += 1
                    }
                    's' => {
                        match chars[i + 1] {
                            'e' => dirs.push(Dir::SE),
                            'w' => dirs.push(Dir::SW),
                            _ => panic!("invalid char sequence {}, {}", chars[i], chars[i + 1]),
                        }
                        i += 1
                    }
                    _ => panic!("invalid char {}", chars[i]),
                }
                i += 1;
            }

            dirs
        })
        .collect();

    let mut black: HashSet<Pos> = HashSet::new();
    for path in paths {
        let pos = path.iter().fold((0, 0), |accum, dir| dir.go(&accum));
        if black.contains(&pos) {
            black.remove(&pos);
        } else {
            black.insert(pos);
        }
    }

    for _ in 0..100 {
        let mut black_adjacents: HashMap<Pos, u32> = HashMap::new();
        for p in black.iter() {
            for other in adjacent(p).iter() {
                *black_adjacents.entry(*other).or_insert(0) += 1;
            }
        }

        let mut next_black = HashSet::new();

        for p in black.iter() {
            let n = *black_adjacents.get(p).unwrap_or(&0);
            if n != 0 && n <= 2 {
                next_black.insert(*p);
            }
        }

        for (p, n) in black_adjacents.iter() {
            if black.contains(p) {
                continue;
            }

            if *n == 2 {
                next_black.insert(*p);
            }
        }

        black = next_black;
    }

    println!("{:?}", black.len());
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
