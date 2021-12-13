use std::io::{self, Read};

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

fn main() {
    let input = get_input();
    let mut lines = input.lines();

    // Phase 1: read in coordinates
    let mut num_rows = 1500;
    let mut num_cols = 1500;
    let mut grid: Vec<Vec<bool>> = vec![vec![false; num_rows]; num_cols];
    loop {
        let line = lines.next().unwrap();
        if line.len() == 0 {
            break;
        }

        let mut toks = line.split(",");
        let row: usize = toks.next().unwrap().parse().unwrap();
        let col: usize = toks.next().unwrap().parse().unwrap();
        grid[col][row] = true;
    }

    // Phase 2: read fold instructions
    let folds: Vec<Fold> = lines
        .map(|line| {
            let line = &line["fold alone ".len()..];
            let mut toks = line.split("=");
            let axis = toks.next().unwrap();
            let num: usize = toks.next().unwrap().parse().unwrap();

            match axis {
                "x" => Fold::X(num),
                "y" => Fold::Y(num),
                _ => panic!("invalid fold instruction: {}", line),
            }
        })
        .collect();

    for f in folds {
        match f {
            Fold::X(num) => {
                for row in 0..num_rows {
                    for col in (num + 1)..num_cols {
                        if !grid[row][col] {
                            continue;
                        }

                        let delta = col - num;
                        grid[row][num - delta] = true;
                    }
                }

                num_cols = num;
            }
            Fold::Y(num) => {
                for row in (num + 1)..num_rows {
                    for col in 0..num_cols {
                        if !grid[row][col] {
                            continue;
                        }

                        let delta = row - num;
                        grid[num - delta][col] = true;
                    }
                }

                num_rows = num;
            }
        }
    }

    for row in 0..num_rows {
        for col in 0..num_cols {
            if grid[row][col] {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!("");
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
