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

fn remove(s: &String, omit: char) -> String {
    s.chars()
        .filter(|c| c.to_ascii_lowercase() != omit)
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input();
    let mut shortest = usize::max_value();

    for i in ('a' as u32)..('z' as u32) {
        let removed = remove(&input, std::char::from_u32(i).unwrap());
        let react_len = react(removed).len();
        if react_len < shortest {
            shortest = react_len
        }
    }

    println!("{}", shortest);
    Ok(())
}
