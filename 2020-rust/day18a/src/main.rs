use std::io::{self, Read};

#[derive(Debug, Copy, Clone)]
enum Operator {
    Plus,
    Multiply,
}

impl Operator {
    fn apply(&self, left: i64, right: i64) -> i64 {
        match self {
            Operator::Plus => left + right,
            Operator::Multiply => left * right,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Token {
    Literal(i64),
    Operator(Operator),
    OpenParens,
    CloseParens,
}

fn tokenize(s: &str) -> Vec<Token> {
    let chars: Vec<char> = s.chars().collect();
    let mut digits: Vec<char> = Vec::new();
    let mut toks = Vec::new();

    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '+' => toks.push(Token::Operator(Operator::Plus)),
            '*' => toks.push(Token::Operator(Operator::Multiply)),
            '(' => toks.push(Token::OpenParens),
            ')' => toks.push(Token::CloseParens),
            n if n.is_ascii_digit() => {
                digits.push(n);
                for j in (i + 1)..chars.len() {
                    if !chars[j].is_ascii_digit() {
                        break;
                    }
                    i += 1;
                    digits.push(chars[j]);
                }
                toks.push(Token::Literal(
                    digits.iter().collect::<String>().parse().unwrap(),
                ));

                digits = Vec::new();
            }
            _ => (),
        }

        i += 1;
    }

    toks
}

fn eval(toks: &[Token]) -> i64 {
    let mut acc = 0;
    let mut cur_op: Option<Operator> = None;
    let mut i = 0;

    while i < toks.len() {
        match toks[i] {
            Token::Literal(n) => match cur_op {
                Some(op) => {
                    acc = op.apply(acc, n);
                    cur_op = None;
                }
                None => {
                    acc = n;
                }
            },

            Token::Operator(op) => {
                assert!(cur_op.is_none());
                cur_op = Some(op);
            }

            Token::OpenParens => {
                // find matching close parens
                let mut open = 1;
                let mut j = i + 1;

                loop {
                    match toks[j] {
                        Token::OpenParens => open += 1,
                        Token::CloseParens => open -= 1,
                        _ => (),
                    }

                    if open == 0 {
                        break;
                    }

                    j += 1;
                }

                if open != 0 {
                    panic!("unclosed parenthesis");
                }

                let n = eval(&toks[(i + 1)..j]);
                match cur_op {
                    Some(op) => {
                        acc = op.apply(acc, n);
                        cur_op = None;
                    }
                    None => acc = n,
                }

                i = j;
            }

            _ => panic!("invalid expr: {:?}", toks),
        }

        i += 1;
    }

    acc
}

fn main() {
    let res = get_input()
        .lines()
        .fold(0, |accum, line| accum + eval(&tokenize(line)));
    println!("{}", res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
