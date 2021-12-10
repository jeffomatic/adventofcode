use std::io::{self, Read};

fn score(c: char) -> i64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
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
    let mut total = 0;
    'next_line: for line in get_input().lines() {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if stack.len() == 0 || c != closing_brace(*stack.last().unwrap()) {
                        total += score(c);
                        continue 'next_line;
                    }
                    stack.pop();
                }
                _ => panic!("unrecognized character {}", c),
            }
        }
    }

    println!("{}", total);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
