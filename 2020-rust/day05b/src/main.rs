use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() {
    let mut ticket_ids: HashSet<u32> = HashSet::new();
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

        ticket_ids.insert(row_min * 8 + col_min);
    }

    let mut sorted: Vec<u32> = ticket_ids.iter().cloned().collect();
    sorted.sort();

    for i in 0..(sorted.len() - 1) {
        let a = sorted[i];
        let b = sorted[i + 1];

        if b - a == 2 {
            println!("{}", b - 1);
            return;
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
