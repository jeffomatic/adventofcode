use std::io::{self, Read};

fn run_east_sim(state: &Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
    let width = state[0].len();

    let mut res = state.clone();
    let mut moved = false;
    for (i, row) in state.iter().enumerate() {
        for (j, &elem) in row.iter().enumerate() {
            if elem != '>' {
                continue;
            }

            let next_col = (j + 1) % width;
            if state[i][next_col] != '.' {
                continue;
            }

            res[i][j] = '.';
            res[i][next_col] = '>';
            moved = true;
        }
    }

    (res, moved)
}

fn run_south_sim(state: &Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
    let height = state.len();

    let mut res = state.clone();
    let mut moved = false;
    for (i, row) in state.iter().enumerate() {
        for (j, &elem) in row.iter().enumerate() {
            if elem != 'v' {
                continue;
            }

            let next_row = (i + 1) % height;
            if state[next_row][j] != '.' {
                continue;
            }

            res[i][j] = '.';
            res[next_row][j] = 'v';
            moved = true;
        }
    }

    (res, moved)
}

fn main() {
    let mut state = parse(&get_input());

    let mut n = 0;
    loop {
        n += 1;

        let east_res = run_east_sim(&state);
        let south_res = run_south_sim(&east_res.0);
        if !east_res.1 && !south_res.1 {
            println!("{}", n);
            return;
        }

        state = south_res.0;
    }
}

fn parse(src: &str) -> Vec<Vec<char>> {
    src.lines().map(|line| line.chars().collect()).collect()
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
