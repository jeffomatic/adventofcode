use std::{
    collections::{HashMap, VecDeque},
    io::{self, Read},
};

fn main() {
    let mut neighbors_of: HashMap<String, Vec<String>> = HashMap::new();
    let input = get_input();
    for line in input.lines() {
        let mut chunks = line.split("-");
        let a = chunks.next().unwrap().to_string();
        let b = chunks.next().unwrap().to_string();
        neighbors_of
            .entry(a.clone())
            .and_modify(|neighbors| neighbors.push(b.clone()))
            .or_insert(vec![b.clone()]);
        neighbors_of
            .entry(b)
            .and_modify(|neighbors| neighbors.push(a.clone()))
            .or_insert(vec![a]);
    }

    let mut q: VecDeque<Vec<String>> = VecDeque::new();
    q.push_back(vec![String::from("start")]);
    let mut num_paths = 0;

    while let Some(path) = q.pop_front() {
        let last = path.last().unwrap();
        for n in neighbors_of.get(last).unwrap().iter() {
            if n.chars().next().unwrap().is_ascii_lowercase() && path.contains(n) {
                continue;
            }

            if n == "end" {
                num_paths += 1;
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(n.clone());
            q.push_back(new_path);
        }
    }

    println!("{}", num_paths);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
