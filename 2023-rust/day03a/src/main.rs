use std::{
    collections::HashSet,
    io::{self, Read},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    i: i32,
    j: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Number {
    val: i32,
    start: Coord,
    len: usize,
}

fn parse(lines: Vec<&str>) -> (Vec<Number>, HashSet<Coord>) {
    let mut numbers = Vec::new();
    let mut symbols = HashSet::new();

    for i in 0..lines.len() {
        let line: Vec<char> = lines[i].chars().collect();
        let mut accum = String::new();
        let mut start = 0;

        for j in 0..line.len() {
            let c = line[j];

            if c == '.' {
                continue;
            }

            if c.is_numeric() {
                if accum.len() == 0 {
                    start = j;
                }

                accum.push(c);
                if j >= line.len() - 1 || !line[j + 1].is_numeric() {
                    numbers.push(Number {
                        val: accum.parse::<i32>().unwrap(),
                        start: Coord {
                            i: i as i32,
                            j: start as i32,
                        },
                        len: j - start + 1,
                    });
                    accum.clear()
                }

                continue;
            }

            symbols.insert(Coord {
                i: i as i32,
                j: j as i32,
            });
        }
    }

    (numbers, symbols)
}

fn has_adj_symbol(n: Number, symbols: &HashSet<Coord>) -> bool {
    for di in -1..=1 {
        for dj in -1..=(n.len as i32) {
            if symbols.contains(&Coord {
                i: n.start.i + di,
                j: n.start.j + dj,
            }) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let input = get_input();
    let (numbers, symbols) = parse(input.split("\n").collect());
    let res: i32 = numbers
        .into_iter()
        .filter(|&n| has_adj_symbol(n, &symbols))
        .map(|n| n.val)
        .sum();
    println!("{}", res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
