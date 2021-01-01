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
    let mut result: u64 = 1;

    for slope in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let dx = slope.0;
        let dy = slope.1;

        let mut x: usize = dx;
        let mut y: usize = dy;
        let mut trees = 0;

        while y < h {
            if map[y % h][x % w] {
                trees += 1
            }

            x += dx;
            y += dy;
        }

        result *= trees;
    }

    println!("{}", result)
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
