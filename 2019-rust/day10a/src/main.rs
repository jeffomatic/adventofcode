use num::integer::gcd;
use std::collections::HashSet;
use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn scan_order(dimensions: (usize, usize), p: (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    for j in p.0..dimensions.0 {
        for i in (0..p.1).rev() {
            res.push((j, i));
        }
    }
    for i in p.1..dimensions.1 {
        for j in (p.0 + 1)..dimensions.0 {
            res.push((j, i));
        }
    }
    for j in (0..=p.0).rev() {
        for i in (p.1 + 1)..dimensions.1 {
            res.push((j, i));
        }
    }
    for i in (0..=p.1).rev() {
        for j in (0..p.0).rev() {
            res.push((j, i));
        }
    }

    return res;
}

fn count_visible(p: (usize, usize), map: &Vec<Vec<bool>>) -> usize {
    let mut slopes = HashSet::new();

    for c in scan_order((map.len(), map[0].len()), p) {
        if !map[c.1][c.0] {
            continue;
        }

        let delta = (c.0 as i64 - p.0 as i64, c.1 as i64 - p.1 as i64);
        let d = gcd(delta.0, delta.1);
        let slope = (delta.0 / d, delta.1 / d);
        slopes.insert(slope);
    }

    return slopes.len();
}

fn main() {
    let input = get_input();
    let map: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("invalid character"),
                })
                .collect()
        })
        .collect();

    let mut best_count = 0;
    let mut best_point = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if !map[i][j] {
                continue;
            }

            let count = count_visible((j, i), &map);
            if count > best_count {
                best_count = count;
                best_point = (j, i);
            }
        }
    }

    println!("{:?} {}", best_point, best_count);
}
