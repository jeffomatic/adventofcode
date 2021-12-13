use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, Read},
};

#[derive(Debug, Clone)]
struct Path<'a> {
    current: &'a str,
    prev_caves: HashSet<&'a str>,
    has_double_lowercase: bool,
}

fn main() {
    let mut neighbors_of: HashMap<&str, Vec<&str>> = HashMap::new();
    let input = get_input();
    for line in input.lines() {
        let mut chunks = line.split("-");
        let a = chunks.next().unwrap();
        let b = chunks.next().unwrap();
        neighbors_of
            .entry(a)
            .and_modify(|neighbors| neighbors.push(b))
            .or_insert(vec![b]);
        neighbors_of
            .entry(b)
            .and_modify(|neighbors| neighbors.push(a))
            .or_insert(vec![a]);
    }

    let mut q: VecDeque<Path> = VecDeque::new();
    q.push_back(Path {
        current: "start",
        prev_caves: HashSet::new(),
        has_double_lowercase: false,
    });

    let mut num_paths = 0;
    while let Some(path) = q.pop_front() {
        for n in neighbors_of.get(&path.current).unwrap().iter() {
            if *n == "start" {
                continue;
            }

            if *n == "end" {
                num_paths += 1;
                continue;
            }

            let mut has_double_lowercase = path.has_double_lowercase;
            if n.chars().next().unwrap().is_ascii_lowercase() {
                if path.prev_caves.contains(n) {
                    if path.has_double_lowercase {
                        continue;
                    }

                    has_double_lowercase = true;
                }
            }

            let mut new_path = Path {
                current: n,
                prev_caves: path.prev_caves.clone(),
                has_double_lowercase,
            };
            new_path.prev_caves.insert(n);

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
