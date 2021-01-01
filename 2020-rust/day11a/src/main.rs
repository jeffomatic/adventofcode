use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() {
    let mut seats: HashSet<(usize, usize)> = HashSet::new();
    for (i, line) in get_input().lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'L' {
                seats.insert((i, j));
            }
        }
    }

    let mut occupied: HashSet<(usize, usize)> = HashSet::new();
    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];

    loop {
        let mut changed = false;
        let mut next_occupied = HashSet::new();

        for s in seats.iter() {
            // Count adjacent occupied seats
            let mut occ_neighbors = 0;
            for (di, dj) in dirs.iter() {
                let i = s.0 as i64 + di;
                if i < 0 {
                    continue;
                }

                let j = s.1 as i64 + dj;
                if j < 0 {
                    continue;
                }

                if occupied.contains(&(i as usize, j as usize)) {
                    occ_neighbors += 1;
                }
            }

            if occupied.contains(s) {
                if occ_neighbors < 4 {
                    next_occupied.insert(*s);
                } else {
                    changed = true;
                }
            } else {
                if occ_neighbors == 0 {
                    next_occupied.insert(*s);
                    changed = true;
                }
            }
        }

        if !changed {
            println!("{}", occupied.len());
            break;
        }

        occupied = next_occupied;
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
