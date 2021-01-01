use std::io::{self, Read};

fn get_input() -> String {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();
    handle.read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}

fn reduce(s: &String) -> String {
    let mut prev = None;
    let mut res = String::new();

    for c in s.chars() {
        match prev {
            Some(p) => {
                if c != p && c.to_ascii_lowercase() == p.to_ascii_lowercase() {
                    prev = None
                } else {
                    res.push(p);
                    prev = Some(c)
                }
            }
            None => prev = Some(c),
        }
    }

    match prev {
        Some(p) => res.push(p),
        None => (),
    }

    res
}

fn react(mut s: String) -> String {
    loop {
        let next = reduce(&s);
        if s.len() == next.len() {
            return s;
        }
        s = next
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current = get_input();
    println!("{}", react(current).len());
    Ok(())
}
