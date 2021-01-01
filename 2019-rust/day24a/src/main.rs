use std::collections::HashSet;

type Grid = [[bool; 5]; 5];

fn new_grid() -> Grid {
    [[false; 5]; 5]
}

fn grid_from_string(s: &str) -> Grid {
    let mut g: Grid = new_grid();
    for (i, line) in s.trim().lines().enumerate() {
        for (j, c) in line.trim().chars().enumerate() {
            g[i][j] = c == '#';
        }
    }
    g
}

fn neighbors(pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let i = pos.0;
    let j = pos.1;

    if 0 < i {
        neighbors.push((i - 1, j));
    }

    if i < 5 - 1 {
        neighbors.push((i + 1, j));
    }

    if 0 < j {
        neighbors.push((i, j - 1));
    }

    if j < 5 - 1 {
        neighbors.push((i, j + 1));
    }
    neighbors
}

fn count_adjacent(g: &Grid, p: (usize, usize)) -> usize {
    neighbors(p).iter().fold(
        0,
        |acc, (n_i, n_j)| if g[*n_i][*n_j] { acc + 1 } else { acc },
    )
}

fn biodiversity_rating(g: &Grid) -> u64 {
    let mut rating = 0;
    for i in 0..5 {
        for j in 0..5 {
            if g[i][j] {
                rating += 1 << (5 * i) + j;
            }
        }
    }
    rating
}

fn simulate(g: &Grid) -> Grid {
    let mut next = new_grid();

    for i in 0..5 {
        for j in 0..5 {
            let adjacent = count_adjacent(g, (i, j));
            next[i][j] = if g[i][j] {
                adjacent == 1
            } else {
                adjacent == 1 || adjacent == 2
            };
        }
    }

    next
}

fn grid_to_string(g: &Grid) -> String {
    g.iter()
        .map(|row| {
            row.iter()
                .map(|v| if *v { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() {
    let input = "
        ##.#.
        .#.##
        .#...
        #..#.
        .##..
    ";

    let mut g = grid_from_string(input);
    let mut seen = HashSet::new();
    loop {
        if seen.contains(&g) {
            println!(
                "{}\nrating: {}",
                grid_to_string(&g),
                biodiversity_rating(&g)
            );
            return;
        }

        seen.insert(g);
        g = simulate(&g);
    }
}
