use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();
    handle.read_to_string(&mut input)?;

    let total = input
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .fold(0, |memo, n| memo + n);

    println!("{}", total);
    Ok(())
}
