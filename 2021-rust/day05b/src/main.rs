use std::{
    collections::HashMap,
    io::{self, Read},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    a: Point,
    b: Point,
}

fn main() {
    let input = get_input();
    let lines: Vec<Line> = input
        .lines()
        .map(|line| {
            let chunks: Vec<&str> = line.split(" -> ").collect();
            let a: Vec<i64> = chunks[0]
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect();
            let b: Vec<i64> = chunks[1]
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect();
            Line {
                a: Point { x: a[0], y: a[1] },
                b: Point { x: b[0], y: b[1] },
            }
        })
        .collect();

    let mut counts: HashMap<Point, i64> = HashMap::new();
    for line in lines.iter() {
        if line.a.x != line.b.x && line.a.y == line.b.y {
            // horizontal line
            let y = line.a.y;
            let min = line.a.x.min(line.b.x);
            let max = line.a.x.max(line.b.x);
            for x in min..=max {
                counts
                    .entry(Point { x, y })
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        } else if line.a.y != line.b.y && line.a.x == line.b.x {
            // vertical line
            let x = line.a.x;
            let min = line.a.y.min(line.b.y);
            let max = line.a.y.max(line.b.y);
            for y in min..=max {
                counts
                    .entry(Point { x, y })
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        } else {
            // diagonals
            let dx = (line.b.x - line.a.x).signum();
            let dy = (line.b.y - line.a.y).signum();
            for i in 0..=(line.b.x - line.a.x).abs() {
                counts
                    .entry(Point {
                        x: line.a.x + i * dx,
                        y: line.a.y + i * dy,
                    })
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    let overlaps = counts.iter().map(|(_, c)| c).filter(|c| **c >= 2).count();
    println!("{}", overlaps);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
