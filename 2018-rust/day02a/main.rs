use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();
    handle.read_to_string(&mut input)?;

    let mut total2 = 0;
    let mut total3 = 0;
    for line in input.lines() {
        let mut counts_by_char = HashMap::new();
        for c in line.chars() {
            let v = match counts_by_char.get(&c) {
                Some(n) => *n,
                None => 0,
            };
            counts_by_char.insert(c, v + 1);
        }

        let counts: HashSet<_> = counts_by_char.values().collect();
        if counts.contains(&2) {
            total2 += 1;
        }
        if counts.contains(&3) {
            total3 += 1;
        }
    }

    println!("{} {} {}", total2, total3, total2 * total3);
    Ok(())
}
