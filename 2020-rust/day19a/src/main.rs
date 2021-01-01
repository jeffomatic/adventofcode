use std::{
    collections::HashMap,
    io::{self, Read},
};

#[derive(Debug, Clone)]
enum Rule<'a> {
    Char(&'a str, char),
    Union(&'a str, Vec<Rule<'a>>),
    Seq(&'a str, Vec<usize>),
}

impl Rule<'_> {
    fn to_string(&self) -> &str {
        match self {
            Self::Char(s, _) => s,
            Self::Union(s, _) => s,
            Self::Seq(s, _) => s,
        }
    }
}

fn check<'a>(
    r: &'a Rule<'a>,
    message: &'a str,
    ruleset: &'a Vec<Rule<'a>>,
    cache: &mut HashMap<(&'a str, &'a str), bool>,
) -> bool {
    if let Some(ok) = cache.get(&(r.to_string(), message)) {
        return *ok;
    }

    let ok = match r {
        Rule::Char(_, c) => message.len() == 1 && message.chars().nth(0).unwrap() == *c,
        Rule::Union(_, rules) => rules.iter().any(|r| check(r, message, ruleset, cache)),
        Rule::Seq(_, seq) => match seq.len() {
            1 => check(&ruleset[seq[0]], message, ruleset, cache),
            2 => {
                let mut ok = false;
                for i in 1..message.len() {
                    if check(&ruleset[seq[0]], &message[0..i], ruleset, cache)
                        && check(&ruleset[seq[1]], &message[i..], ruleset, cache)
                    {
                        ok = true;
                        break;
                    }
                }
                ok
            }
            _ => panic!("unsupported sequence length {}", seq.len()),
        },
    };

    cache.insert((r.to_string(), message), ok);

    ok
}

fn parse_rule(def: &str) -> Rule {
    if let Some(pos) = def.chars().position(|c| c == '"') {
        Rule::Char(def, def.chars().nth(pos + 1).unwrap())
    } else if def.contains("|") {
        Rule::Union(def, def.split(" | ").map(|seq| parse_rule(seq)).collect())
    } else {
        Rule::Seq(
            def,
            def.split(" ").map(|tok| tok.parse().unwrap()).collect(),
        )
    }
}

fn main() {
    let input = get_input();
    let sections: Vec<&str> = input.split("\n\n").collect();
    let (rule_input, message_input) = (sections[0], sections[1]);

    let mut rules: Vec<Rule> = vec![Rule::Char("", '\0'); rule_input.lines().count()];
    for line in rule_input.lines() {
        let chunks: Vec<&str> = line.split(": ").collect();
        let (num, def) = (chunks[0], chunks[1]);
        rules[num.parse::<usize>().unwrap()] = parse_rule(def);
    }

    let messages: Vec<&str> = message_input.lines().collect();
    let mut cache = HashMap::new();

    let res = messages
        .iter()
        .filter(|message| rules.iter().any(|r| check(r, message, &rules, &mut cache)))
        .count();
    println!("{}", res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
