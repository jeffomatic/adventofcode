use std::collections::HashSet;

type Pos = (i64, usize, usize); // depth, row, column

fn bugset_from_string(s: &str) -> HashSet<Pos> {
    let mut bugs = HashSet::new();
    for (i, line) in s.trim().lines().enumerate() {
        for (j, c) in line.trim().chars().enumerate() {
            if c == '#' {
                bugs.insert((0, i, j));
            }
        }
    }
    bugs
}

fn neighbors(pos: Pos) -> Vec<Pos> {
    let mut neighbors = Vec::new();
    let (depth, i, j) = pos;

    // Neighbors at same depth
    if 0 < i && (i, j) != (3, 2) {
        neighbors.push((depth, i - 1, j));
    }

    if i < 4 && (i, j) != (1, 2) {
        neighbors.push((depth, i + 1, j));
    }

    if 0 < j && (i, j) != (2, 3) {
        neighbors.push((depth, i, j - 1));
    }

    if j < 4 && (i, j) != (2, 1) {
        neighbors.push((depth, i, j + 1));
    }

    // Neighbors at lower level of recursion
    if (i, j) == (1, 2) {
        for j in 0..5 {
            neighbors.push((depth + 1, 0, j));
        }
    }

    if (i, j) == (3, 2) {
        for j in 0..5 {
            neighbors.push((depth + 1, 4, j));
        }
    }

    if (i, j) == (2, 1) {
        for i in 0..5 {
            neighbors.push((depth + 1, i, 0));
        }
    }

    if (i, j) == (2, 3) {
        for i in 0..5 {
            neighbors.push((depth + 1, i, 4));
        }
    }

    // Neighbors at higher level of recursion
    if i == 0 {
        neighbors.push((depth - 1, 1, 2));
    }

    if i == 4 {
        neighbors.push((depth - 1, 3, 2));
    }

    if j == 0 {
        neighbors.push((depth - 1, 2, 1));
    }

    if j == 4 {
        neighbors.push((depth - 1, 2, 3));
    }

    neighbors
}

fn count_adjacent(bugs: &HashSet<Pos>, p: Pos) -> usize {
    neighbors(p)
        .iter()
        .fold(0, |acc, n| if bugs.contains(n) { acc + 1 } else { acc })
}

fn points_of_interest(bugs: &HashSet<Pos>) -> HashSet<Pos> {
    let mut ws = bugs.clone();
    for p in bugs.iter() {
        for n in neighbors(*p).iter() {
            ws.insert(*n);
        }
    }
    ws
}

fn simulate(bugs: &HashSet<Pos>) -> HashSet<Pos> {
    let mut next = HashSet::new();

    for p in points_of_interest(bugs).iter() {
        let adjacent = count_adjacent(bugs, *p);
        if bugs.contains(p) {
            if adjacent == 1 {
                next.insert(*p);
            }
        } else {
            if adjacent == 1 || adjacent == 2 {
                next.insert(*p);
            }
        }
    }

    next
}

fn main() {
    // examples
    println!(
        "tile 19: {} neighbors: {:?}",
        neighbors((0, 3, 3)).len(),
        neighbors((0, 3, 3))
    );
    println!(
        "tile G: {} neighbors: {:?}",
        neighbors((1, 1, 1)).len(),
        neighbors((1, 1, 1))
    );
    println!(
        "tile D: {} neighbors: {:?}",
        neighbors((1, 0, 3)).len(),
        neighbors((1, 0, 3))
    );
    println!(
        "tile E: {} neighbors: {:?}",
        neighbors((1, 0, 4)).len(),
        neighbors((1, 0, 4))
    );
    println!(
        "tile 14: {} neighbors: {:?}",
        neighbors((0, 2, 3)).len(),
        neighbors((0, 2, 3))
    );
    println!(
        "tile N: {} neighbors: {:?}",
        neighbors((1, 2, 3)).len(),
        neighbors((1, 2, 3))
    );

    let input = "
        ##.#.
        .#.##
        .#...
        #..#.
        .##..
    ";

    println!(
        "{}",
        (0..200)
            .fold(bugset_from_string(input), |bugs, _| simulate(&bugs))
            .len()
    );
}
