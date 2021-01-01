use std::{
    collections::HashMap,
    io::{self, Read},
};

fn main() {
    let input = get_input();
    let mut total = 0;

    for chunk in input.split("\n\n") {
        let mut yes: HashMap<char, usize> = HashMap::new();
        let mut group_size: usize = 0;

        for line in chunk.lines() {
            group_size += 1;

            for c in line.chars() {
                if yes.contains_key(&c) {
                    *yes.get_mut(&c).unwrap() += 1;
                } else {
                    yes.insert(c, 1);
                }
            }
        }

        total += yes.values().filter(|&&v| v == group_size).count();
    }

    println!("{}", total);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
