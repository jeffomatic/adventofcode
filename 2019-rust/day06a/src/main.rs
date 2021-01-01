use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let input = get_input();
    let mut parents: HashMap<&str, &str> = HashMap::new();
    let mut all_children = HashSet::new();

    for line in input.lines() {
        let toks: Vec<&str> = line.split(")").collect();
        let parent = toks[0];
        let child = toks[1];
        all_children.insert(child);
        parents.insert(child, parent);
    }

    let mut orbits = 0;
    for c in all_children.iter() {
        let mut cur = c;
        loop {
            match parents.get(cur) {
                None => break,
                Some(p) => {
                    orbits += 1;
                    cur = p;
                }
            }
        }
    }

    println!("{:?}", orbits);
}
