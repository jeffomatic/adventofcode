use std::io::{self, Read};

fn neighbors(i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    for di in -1..=1 {
        // ignore out-of-bounds rows
        let cur_i = (i as i32) + di;
        if cur_i < 0 || 10 <= cur_i {
            continue;
        }

        for dj in -1..=1 {
            // ignore self
            if di == 0 && dj == 0 {
                continue;
            }

            // ignore out-of-bounds columns
            let cur_j = (j as i32) + dj;
            if cur_j < 0 || 10 <= cur_j {
                continue;
            }

            res.push((cur_i as usize, cur_j as usize));
        }
    }

    return res;
}

fn main() {
    let mut grid: Vec<Vec<i8>> = get_input()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect();

    let mut step = 0;
    loop {
        step += 1;

        // add energy and record all flashes
        let mut q: Vec<(usize, usize)> = Vec::new();
        for (i, row) in grid.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                *col += 1;
                if *col > 9 {
                    q.push((i, j));
                }
            }
        }

        // handle flashes
        let mut step_flashes = 0;
        while q.len() > 0 {
            let (i, j) = q.pop().unwrap();

            grid[i][j] = 0;
            step_flashes += 1;

            for (ni, nj) in neighbors(i, j) {
                // has the neighbor already flashed or been pushed into the list?
                if grid[ni][nj] == 0 || grid[ni][nj] > 9 {
                    continue;
                }

                grid[ni][nj] += 1;
                if grid[ni][nj] > 9 {
                    q.push((ni, nj));
                }
            }
        }

        if step_flashes == 100 {
            println!("{}", step);
            return;
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
