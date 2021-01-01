use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();
    handle.read_to_string(&mut input)?;
    let values: Vec<i32> = input.lines().map(|s| s.parse::<i32>().unwrap()).collect();

    let mut total = 0;
    let mut seen = HashSet::new();
    loop {
        for n in &values {
            if seen.contains(&total) {
                println!("{}", total);
                return Ok(());
            }

            seen.insert(total);
            total += n;
        }
    }
}
