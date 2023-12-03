use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    i: i32,
    j: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Number {
    start: Coord,
    val: i32,
}

fn parse(lines: Vec<&str>) -> (HashMap<Coord, Number>, Vec<Coord>) {
    let mut nums_by_pos = HashMap::new();
    let mut gears = Vec::new();
    let h = lines.len() as i32;
    let w = lines.first().unwrap().len() as i32;

    for i in 0..h {
        let line: Vec<char> = lines[i as usize].chars().collect();
        let mut accum = String::new();
        let mut start_j = 0;

        for j in 0..w {
            let c = line[j as usize];

            if c == '*' {
                gears.push(Coord { i, j });
                continue;
            }

            if !c.is_numeric() {
                continue;
            }

            if accum.len() == 0 {
                start_j = j;
            }

            accum.push(c);

            if j >= w - 1 || !line[j as usize + 1].is_numeric() {
                let val = accum.parse::<i32>().unwrap();
                let number = Number {
                    start: Coord { i, j: start_j },
                    val,
                };
                let len = j - start_j + 1;
                for dj in 0..len {
                    nums_by_pos.insert(Coord { i, j: start_j + dj }, number);
                }
                accum.clear()
            }
        }
    }

    (nums_by_pos, gears)
}

fn main() {
    let input = get_input();
    let (numbers, gears) = parse(input.split("\n").collect());

    let mut neighbors = HashSet::new();
    let mut res = 0;
    for g in gears {
        for di in -1..=1 {
            for dj in -1..=1 {
                let c = Coord {
                    i: g.i + di,
                    j: g.j + dj,
                };
                if let Some(&num) = numbers.get(&c) {
                    neighbors.insert(num);
                }
            }
        }

        if neighbors.len() == 2 {
            res += neighbors.iter().fold(1, |acc, n| acc * n.val);
        }

        neighbors.clear();
    }

    println!("{}", res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
