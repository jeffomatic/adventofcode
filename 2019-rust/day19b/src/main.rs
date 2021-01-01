use intcode;
use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn check_location(program: &Vec<i64>, pos: (i64, i64)) -> bool {
    intcode::Computer::new(&program)
        .run(&vec![pos.0, pos.1])
        .output[0]
        == 1
}

fn get_first_left_of(program: &Vec<i64>, pos: (i64, i64), state: bool) -> i64 {
    let mut x = pos.0;
    while check_location(program, (x, pos.1)) != state {
        x -= 1;
    }
    x
}

fn get_first_right_of(program: &Vec<i64>, pos: (i64, i64), state: bool) -> i64 {
    let mut x = pos.0;
    while check_location(program, (x, pos.1)) != state {
        x += 1;
    }
    x
}

fn get_row_range(program: &Vec<i64>, y: i64, prev_range: (i64, i64)) -> (i64, i64) {
    let min_x = if check_location(program, (prev_range.0, y)) {
        get_first_left_of(program, (prev_range.0, y), false) + 1
    } else {
        get_first_right_of(program, (prev_range.0, y), true)
    };

    let max_x = if check_location(program, (prev_range.1, y)) {
        get_first_right_of(program, (prev_range.1, y), false) - 1
    } else {
        get_first_left_of(program, (prev_range.1, y), true)
    };

    (min_x, max_x)
}

fn overlap(a: (i64, i64), b: (i64, i64)) -> i64 {
    std::cmp::max(0, std::cmp::min(a.1, b.1) - std::cmp::max(a.0, b.0) + 1)
}

fn main() {
    let program: Vec<i64> = get_input().split(",").map(|s| s.parse().unwrap()).collect();

    let want_size = 100;
    let mut range = (28, 35);
    let mut y = 50;
    let mut history = Vec::new();

    loop {
        range = get_row_range(&program, y, range);

        if (range.1 - range.0 + 1) < want_size {
            history = Vec::new();
        } else {
            // scan backward and break history at any past range that doesn't
            // share a want_size-length overlap with the current range
            for i in (0..history.len()).rev() {
                let other = history[i];
                if overlap(other, range) < want_size {
                    history = history.split_off(i + 1);
                    break;
                }
            }

            history.push(range);
        }

        if history.len() as i64 == want_size {
            break;
        }

        y += 1;
    }

    let start_x = range.0;
    let start_y = y - want_size + 1;
    println!("({}, {}) {}", start_x, start_x, 10000 * start_x + start_y);
}
