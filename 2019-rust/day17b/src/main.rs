use intcode;
use std::io::{self, Read};

#[derive(Copy, Clone, Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

#[derive(Copy, Clone, Debug)]
enum Turn {
    Left,
    Right,
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

    fn turn(&self, t: Turn) -> Dir {
        match t {
            Turn::Left => match self {
                Dir::N => Dir::W,
                Dir::S => Dir::E,
                Dir::E => Dir::N,
                Dir::W => Dir::S,
            },
            Turn::Right => match self {
                Dir::N => Dir::E,
                Dir::S => Dir::W,
                Dir::E => Dir::S,
                Dir::W => Dir::N,
            },
        }
    }

    fn move_from(&self, p: (i64, i64)) -> (i64, i64) {
        match self {
            Dir::N => (p.0, p.1 - 1),
            Dir::S => (p.0, p.1 + 1),
            Dir::E => (p.0 + 1, p.1),
            Dir::W => (p.0 - 1, p.1),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct BotState {
    pos: (i64, i64),
    orientation: Dir,
}

// returns a tuple of:
// - a 2D vector of bools, where true means the location of a scaffold
// - the current state of the bot
fn parse_map(data: &Vec<i64>) -> (Vec<Vec<bool>>, BotState) {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut bot: Option<BotState> = None;
    let mut i = 0;
    let mut j = 0;

    for c in data.iter() {
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

fn on_scaffold(map: &Vec<Vec<bool>>, p: (i64, i64)) -> bool {
    let h = map.len() as i64;
    let w = map[0].len() as i64;
    0 <= p.0 && p.0 < w && 0 <= p.1 && p.1 < h && map[p.1 as usize][p.0 as usize]
}

fn path_commands(map: &Vec<Vec<bool>>, bot: BotState) -> Vec<String> {
    let mut path: Vec<String> = Vec::new();
    let mut bot = bot.clone();

    loop {
        // Scan forward
        let mut steps = 0;
        loop {
            let p = bot.orientation.move_from(bot.pos);
            if !on_scaffold(map, p) {
                if steps > 0 {
                    path.push(steps.to_string());
                }
                break;
            }

            bot.pos = p;
            steps += 1;
        }

        // Check for left turn
        if on_scaffold(map, bot.orientation.turn(Turn::Left).move_from(bot.pos)) {
            bot.orientation = bot.orientation.turn(Turn::Left);
            path.push("L".to_string());
            continue;
        }

        // Check for right turn
        if on_scaffold(map, bot.orientation.turn(Turn::Right).move_from(bot.pos)) {
            bot.orientation = bot.orientation.turn(Turn::Right);
            path.push("R".to_string());
            continue;
        }

        return path;
    }
}

fn subseq_index_of<T: Eq>(from: &[T], sub: &[T], starting_at: usize) -> Option<usize> {
    'next_start: for i in starting_at..(from.len() - sub.len() + 1) {
        for j in 0..sub.len() {
            if from[i + j] != sub[j] {
                continue 'next_start;
            }
        }

        return Some(i);
    }

    None
}

fn subseq_remove<T: Eq + Clone>(from: &[T], sub: &[T]) -> Vec<T> {
    let mut res = Vec::new();
    let mut cursor = 0;

    loop {
        match subseq_index_of(from, sub, cursor) {
            None => {
                // If no subsequences remain, copy the rest of the source to
                // the result.
                for i in cursor..from.len() {
                    res.push(from[i].clone());
                }
                return res;
            }
            Some(n) => {
                // If there is an instance of the subsequence, copy everything
                // from the current search cursor to the start of the subsequence.
                // Then, continue searching after the subsequence.
                for i in cursor..n {
                    res.push(from[i].clone());
                }
                cursor = n + sub.len();
            }
        }
    }
}

fn get_largest_repeated_prefix<'a>(
    from: &'a [String],
    max_joined_len: usize,
) -> Option<&'a [String]> {
    for w in (1..(from.len() / 2)).rev() {
        let prefix = &from[0..w];
        if prefix.join(",").len() > max_joined_len {
            continue;
        }

        if let Some(_) = subseq_index_of(from, prefix, prefix.len()) {
            return Some(prefix);
        }
    }

    None
}

fn decompose_into_subseqs(
    seq: &[String],
    max_subseqs: usize,
    max_subseq_len: usize,
) -> Option<Vec<Vec<String>>> {
    if max_subseqs == 0 {
        return None;
    }

    if seq.join(",").len() <= max_subseq_len {
        return Some(vec![seq.to_vec()]);
    }

    // Greedily attempt to use the longest possible repeated prefix to generate
    // further subsequences. If we can't find a set of subsequences that fits
    // within max_subseqs, we should shorten the size of the prefix and try again.
    let mut max_prefix_len = max_subseq_len;
    while max_prefix_len > 0 {
        match get_largest_repeated_prefix(seq, max_prefix_len) {
            None => return None,
            Some(prefix) => {
                let without_prefix_subseq = subseq_remove(seq, prefix);

                // Here, we make an assumption that every subsequence of the
                // top-level sequence is repeated at least once. If we didn't
                // make that assumption, we'd need to split a sequence into
                // multiple non-repeating sequences based on length, which is
                // not something that's necessary for this problem input.
                if without_prefix_subseq.is_empty() {
                    return Some(vec![prefix.to_vec()]);
                }

                if let Some(mut subseqs) = decompose_into_subseqs(
                    without_prefix_subseq.as_slice(),
                    max_subseqs - 1,
                    max_subseq_len,
                ) {
                    if subseqs.len() < max_subseqs {
                        let mut result = vec![prefix.to_vec()];
                        result.append(&mut subseqs);
                        return Some(result);
                    }
                }

                max_prefix_len -= 1;
            }
        }
    }

    None
}

fn decompose_to_subseq_indexes(seq: &[String], subseqs: &Vec<Vec<String>>) -> Option<Vec<usize>> {
    for (i, prefix) in subseqs.iter().enumerate() {
        if seq.starts_with(prefix) {
            let (_, without_prefix) = seq.split_at(prefix.len());
            let mut result = vec![i];

            if without_prefix.is_empty() {
                return Some(result);
            }

            if let Some(mut indexes) = decompose_to_subseq_indexes(without_prefix, subseqs) {
                result.append(&mut indexes);
                return Some(result);
            }
        }
    }

    None
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut program: Vec<i64> = get_input().split(",").map(|s| s.parse().unwrap()).collect();
    let result = intcode::Computer::new(&program).run(&Vec::new());
    let (map, bot) = parse_map(&result.output);

    let path = path_commands(&map, bot);
    println!("Scaffold path: {}", path.join(","));

    let max_subseqs = 3;
    let max_subseq_len = 20;
    let subseqs = decompose_into_subseqs(path.as_slice(), max_subseqs, max_subseq_len).unwrap();
    println!("Repeated subsequences: {:?}", subseqs);

    let subseq_indexes = decompose_to_subseq_indexes(path.as_slice(), &subseqs).unwrap();
    println!("Sequence as subsequence indexes: {:?}", subseq_indexes);

    let main_routine = subseq_indexes
        .iter()
        .map(|n| match n {
            0 => "A",
            1 => "B",
            2 => "C",
            unknown => panic!("invalid subseuqnce index {}", unknown),
        })
        .collect::<Vec<&str>>()
        .join(",");

    let mut input_lines = vec![main_routine];
    for s in subseqs.iter() {
        input_lines.push(s.join(","));
    }
    input_lines.push("n".to_string());
    let input_str = input_lines.join("\n") + "\n";

    program[0] = 2; // set program to manual control
    let input: Vec<i64> = input_str.chars().map(|c| c as i64).collect();
    let result = intcode::Computer::new(&program).run(&input);
    println!("{}", result.output.last().unwrap());
}
