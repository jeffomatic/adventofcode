use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

type Pos = (i64, i64, i64, i64);

fn main() {
    let mut active: HashSet<Pos> = HashSet::new();
    for (y, line) in get_input().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active.insert((x as i64, y as i64, 0, 0));
            }
        }
    }

    for _ in 0..6 {
        let mut next_active: HashSet<Pos> = HashSet::new();
        let mut neighbor_count: HashMap<Pos, usize> = HashMap::new();
        for (x, y, z, w) in active.iter() {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        for dw in -1..=1 {
                            if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                                let other = (x + dx, y + dy, z + dz, w + dw);
                                *neighbor_count.entry(other).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }

        // Surival check
        for p in active.iter() {
            match neighbor_count.get(p) {
                Some(2) | Some(3) => {
                    next_active.insert(*p);
                }
                _ => (),
            }
        }

        // Spawn check
        for (pos, active_neighbors) in neighbor_count.iter() {
            if active.contains(pos) {
                continue;
            }
            if *active_neighbors == 3 {
                next_active.insert(*pos);
            }
        }

        active = next_active;
    }

    println!("{}", active.len());
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
