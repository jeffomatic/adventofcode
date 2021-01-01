use intcode;
use std::collections::HashSet;
use std::io::{self, Read};

#[derive(Copy, Clone, Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn from_char(c: char) -> Option<Dir> {
        match c {
            '^' => Some(Dir::N),
            'v' => Some(Dir::S),
            '>' => Some(Dir::E),
            '<' => Some(Dir::W),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Dir::N => '^',
            Dir::S => 'v',
            Dir::E => '>',
            Dir::W => '<',
        }
    }
}

struct BotState {
    pos: (usize, usize),
    orientation: Dir,
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn render(rows: &Vec<Vec<bool>>, bot: &BotState) {
    for (i, row) in rows.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if (j, i) == bot.pos {
                print!("{}", bot.orientation.to_char());
                continue;
            }
            print!("{}", if *col { '#' } else { '.' });
        }
        print!("\n");
    }
}

fn get_intersections(rows: &Vec<Vec<bool>>) -> HashSet<(usize, usize)> {
    let h = rows.len();
    let w = rows[0].len();
    let mut result = HashSet::new();

    // can't have intersections at the top or bottom row
    for i in 1..(h - 1) {
        // can't have intersections at the left or right columns
        for j in 1..(w - 1) {
            if rows[i][j] && rows[i - 1][j] && rows[i + 1][j] && rows[i][j - 1] && rows[i][j + 1] {
                result.insert((j, i));
            }
        }
    }

    return result;
}

// returns a tuple of:
// - a 2D vector of bools, where true means the location of a scaffold
// - the current state of the bot
fn parse_output(output: &Vec<i64>) -> (Vec<Vec<bool>>, BotState) {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut bot: Option<BotState> = None;
    let mut i = 0;
    let mut j = 0;

    for c in output.iter() {
        let c = *c as u8 as char;

        if c == '\n' {
            if row.len() > 0 {
                rows.push(row);
                i += 1;
            }

            row = Vec::new();
            j = 0;

            continue;
        }

        row.push(c != '.');

        if let Some(dir) = Dir::from_char(c) {
            bot = Some(BotState {
                pos: (j, i),
                orientation: dir,
            });
        }

        j += 1;
    }

    (rows, bot.unwrap())
}

fn main() {
    let program: Vec<i64> = get_input().split(",").map(|s| s.parse().unwrap()).collect();
    let result = intcode::Computer::new(&program).run(&Vec::new());
    let parsed = parse_output(&result.output);
    render(&parsed.0, &parsed.1);
    println!(
        "{:?}",
        get_intersections(&parsed.0)
            .iter()
            .map(|p| p.0 * p.1)
            .fold(0, |acc, v| acc + v)
    );
}
