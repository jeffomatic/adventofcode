use std::io::{self, Read};

fn main() {
    let mut map: Vec<Vec<bool>> = Vec::new();
    for line in get_input().lines() {
        let mut row: Vec<bool> = Vec::new();
        for c in line.chars() {
            row.push(c == '#');
        }
        map.push(row)
    }

    let h = map.len();
    let w = map[0].len();
    let mut j: usize = 3;
    let mut trees = 0;
    for i in 1..map.len() {
        if map[i % h][j % w] {
            trees += 1
        }
        j += 3
    }

    println!("{}", trees)
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
