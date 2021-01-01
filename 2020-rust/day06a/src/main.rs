use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() {
    let input = get_input();
    let mut total = 0;

    for chunk in input.split("\n\n") {
        let mut yes: HashSet<char> = HashSet::new();
        for line in chunk.lines() {
            for c in line.chars() {
                yes.insert(c);
            }
        }
        total += yes.len();
    }

    println!("{}", total);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
