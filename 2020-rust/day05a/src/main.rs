use std::io::{self, Read};

fn main() {
    let mut best = -1;
    for line in get_input().lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut row_min = 0;
        let mut row_max = 127;

        for i in 0..7 {
            match chars[i] {
                'F' => row_max = row_min + (row_max - row_min) / 2,
                'B' => row_min = row_min + (row_max - row_min) / 2 + 1,
                _ => unreachable!(),
            };
        }

        let mut col_min = 0;
        let mut col_max = 7;
        for i in 7..10 {
            match chars[i] {
                'L' => col_max = col_min + (col_max - col_min) / 2,
                'R' => col_min = col_min + (col_max - col_min) / 2 + 1,
                _ => unreachable!(),
            };
        }

        let seat_id = row_min * 8 + col_min;
        if seat_id > best {
            best = seat_id;
        }
    }

    println!("{}", best);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
