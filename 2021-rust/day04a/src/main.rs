use std::io::{self, Read};

#[derive(Debug, Clone, Copy)]
struct Board {
    elements: [i64; 25],
}

impl Board {
    fn mark(&mut self, val: i64) {
        for i in 0..self.elements.len() {
            if self.elements[i] == val {
                self.elements[i] = -1;
                break;
            }
        }
    }

    fn has_won(&self) -> bool {
        // check row winners
        'row: for row in 0..5 {
            for col in 0..5 {
                if self.elements[row * 5 + col] != -1 {
                    continue 'row;
                }
            }

            return true;
        }

        // check col winners
        'col: for col in 0..5 {
            for row in 0..5 {
                if self.elements[row * 5 + col] != -1 {
                    continue 'col;
                }
            }

            return true;
        }

        return false;
    }

    fn sum_unmarked(&self) -> i64 {
        self.elements
            .iter()
            .fold(0, |accum, val| if *val != -1 { accum + val } else { accum })
    }
}

fn main() {
    let input = get_input();
    let mut lines: Vec<&str> = input.lines().collect();
    lines.push("");

    let (first, rest) = lines.split_at(2);

    let numbers: Vec<i64> = first[0].split(',').map(|s| s.parse().unwrap()).collect();

    let mut boards: Vec<Board> = Vec::new();
    let mut a: &[&str];
    let mut b = rest;

    while b.len() > 0 {
        let vals = b.split_at(6);
        a = vals.0;
        b = vals.1;

        let mut board = Board { elements: [0; 25] };

        for row in 0..5 {
            for (col, num_str) in a[row].trim().split_ascii_whitespace().enumerate() {
                board.elements[row * 5 + col] = num_str.parse().unwrap();
            }
        }

        boards.push(board);
    }

    for num in numbers.iter() {
        for b in boards.iter_mut() {
            b.mark(*num);
            if b.has_won() {
                println!("num: {}\n{:?}", num, b);
                println!("{}", b.sum_unmarked() * num);
                return;
            }
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
