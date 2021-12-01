use std::io::{self, Read};

fn main() {
    println!("{}", get_input());
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
