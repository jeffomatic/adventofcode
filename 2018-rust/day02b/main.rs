use std::io::{self, Read};

fn get_input() -> String {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();
    handle.read_to_string(&mut input).unwrap();
    return input;
}

fn common_string(a: &[u8], b: &[u8]) -> Option<String> {
    if a.len() != b.len() {
        return None;
    }

    let mut different = false;
    let mut s = String::new();
    for i in 0..a.len() {
        if a[i] != b[i] {
            if different {
                return None;
            }

            different = true;
            continue;
        }

        s.push(a[i].into());
    }

    return Some(s);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input();
    let lines: Vec<_> = input.lines().map(|s| s.as_bytes()).collect();

    for i in 0..(lines.len() - 1) {
        for j in (i + 1)..lines.len() {
            if let Some(s) = common_string(lines[i], lines[j]) {
                println!("{}", s);
                return Ok(());
            }
        }
    }

    Err(From::from("no result"))
}
