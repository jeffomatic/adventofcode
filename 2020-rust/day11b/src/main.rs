use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() {
    let input: Vec<String> = get_input().lines().map(|s| s.to_string()).collect();
    let height = input.len();
    let width = input[0].len();

    let mut seats: HashSet<(usize, usize)> = HashSet::new();
    for (i, line) in input.iter().enumerate() {
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
            let mut occupied_visible = 0;
            for (di, dj) in dirs.iter() {
                let mut i = s.0 as i32;
                let mut j = s.1 as i32;

                loop {
                    i += di;
                    j += dj;

                    if i < 0 || (height as i32) <= i || j < 0 || (width as i32) <= j {
                        break;
                    }

                    let p = (i as usize, j as usize);
                    if seats.contains(&p) {
                        if occupied.contains(&p) {
                            occupied_visible += 1;
                        }
                        break;
                    }
                }
            }

            if occupied.contains(s) {
                if occupied_visible < 5 {
                    next_occupied.insert(*s);
                } else {
                    changed = true;
                }
            } else {
                if occupied_visible == 0 {
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
