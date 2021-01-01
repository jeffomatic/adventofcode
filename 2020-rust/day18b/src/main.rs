use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Debug, Copy, Clone)]
enum Operator {
    Plus,
    Multiply,
}

impl Operator {
    fn apply(&self, left: i64, right: i64) -> i64 {
        match self {
            Self::Plus => left + right,
            Self::Multiply => left * right,
        }
    }

    fn maybe(t: &Token) -> Option<Self> {
        match t {
            Token::Operator(op) => Some(*op),
            _ => None,
        }
    }

    fn precedence(&self) -> i64 {
        match self {
            Self::Plus => 100,
            Self::Multiply => 10,
        }
    }

    fn cmp_precedence(&self, other: &Self) -> Ordering {
        self.precedence().cmp(&other.precedence())
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

enum Node {
    Operation {
        op: Operator,
        left: Box<Node>,
        right: Box<Node>,
    },
    Parenthetical(Box<Node>),
    Leaf(i64),
}

fn parse(toks: &[Token]) -> Node {
    if toks.is_empty() {
        panic!("empty token stream");
    }

    let (first, rest) = toks.split_first().unwrap();
    if Operator::maybe(first).is_some() {
        panic!("operator found at beginning of token stream. unary operators not supported.")
    }

    let (left, rest) = match first {
        Token::CloseParens => panic!("unmatched close brace"),
        Token::OpenParens => parse_parenthetical(rest),
        Token::Literal(n) => (Node::Leaf(*n), rest),
        Token::Operator(_) => panic!("invalid operator"),
    };

    match rest.split_first() {
        None => left,
        Some((op, rest)) => match Operator::maybe(op) {
            None => panic!("operator expected after leaf node"),
            Some(op) => compose_with_precedence(op, left, parse(rest)),
        },
    }
}

// Parses a token stream assuming that the preceding token was an open brace.
// Returns the first node in the stream (the parsed parenthetical expression), plus
// any remaining tokens in the stream not part of the parenthetical expression.
fn parse_parenthetical(toks: &[Token]) -> (Node, &[Token]) {
    let mut open = 1;
    let mut close_pos = None;

    // search for matching closing brace
    for (i, s) in toks.iter().enumerate() {
        match s {
            Token::OpenParens => open += 1,
            Token::CloseParens => {
                open -= 1;
                if open == 0 {
                    close_pos = Some(i);
                    break;
                }
            }
            _ => (),
        }
    }

    match close_pos {
        None => panic!("unmatched open brace"),
        Some(close_pos) => {
            let (inner, after) = toks.split_at(close_pos);
            (
                Node::Parenthetical(Box::new(parse(inner))),
                after.split_first().unwrap().1, // skip closing brace
            )
        }
    }
}

fn compose_with_precedence(op: Operator, left: Node, into: Node) -> Node {
    match into {
        Node::Operation {
            op: subnode_op,
            left: subnode_left,
            right: subnode_right,
        } if op.cmp_precedence(&subnode_op) == Ordering::Greater => Node::Operation {
            op: subnode_op,
            left: Box::new(compose_with_precedence(op, left, *subnode_left)),
            right: subnode_right,
        },
        // operations with equal/lower precedence, leaves, and parentheticals
        _ => Node::Operation {
            op,
            left: Box::new(left),
            right: Box::new(into),
        },
    }
}

fn eval(node: &Node) -> i64 {
    match node {
        Node::Leaf(n) => *n,
        Node::Parenthetical(node) => eval(&node),
        Node::Operation { op, left, right } => op.apply(eval(left), eval(right)),
    }
}

fn main() {
    let res = get_input()
        .lines()
        .fold(0, |accum, line| accum + eval(&parse(&tokenize(line))));
    println!("{}", res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
