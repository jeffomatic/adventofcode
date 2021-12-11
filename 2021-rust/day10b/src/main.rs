use std::io::{self, Read};

fn score(c: char) -> i64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("unrecognized character {}", c),
    }
}

fn closing_brace(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unrecognized character {}", c),
    }
}

fn main() {
    let mut scores: Vec<i64> = Vec::new();
    'next_line: for line in get_input().lines() {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if stack.len() == 0 || c != closing_brace(*stack.last().unwrap()) {
                        continue 'next_line;
                    }
                    stack.pop();
                }
                _ => panic!("unrecognized character {}", c),
            }
        }

        stack.reverse();
        let linescore = stack
            .iter()
            .fold(0, |accum, c| (accum * 5) + score(closing_brace(*c)));

        scores.push(linescore);
    }

    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
