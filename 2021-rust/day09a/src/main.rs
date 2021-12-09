use std::io::{self, Read};

fn main() {
    let height_map: Vec<Vec<i64>> = get_input()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect()
        })
        .collect();

    let num_rows = height_map.len();
    let num_cols = height_map[0].len();

    let mut low_points: Vec<i64> = Vec::new();
    for row in 0..num_rows {
        for col in 0..num_cols {
            let height = height_map[row][col];

            // upper neighbor
            if row > 0 && height >= height_map[row - 1][col] {
                continue;
            }

            // lower neighbor
            if row < num_rows - 1 && height >= height_map[row + 1][col] {
                continue;
            }

            // left neighbor
            if col > 0 && height >= height_map[row][col - 1] {
                continue;
            }

            // right neighbor
            if col < num_cols - 1 && height >= height_map[row][col + 1] {
                continue;
            }

            low_points.push(height);
        }
    }

    println!("{}", low_points.iter().map(|h| h + 1).sum::<i64>())
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
