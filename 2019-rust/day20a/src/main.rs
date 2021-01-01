use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use std::io::{self, Read};

mod map;

#[derive(Clone, Eq, PartialEq)]
struct Path {
    path: Vec<map::Pos>,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        // flip order for maxheap
        other.path.len().cmp(&self.path.len())
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn shortest_path(map: &map::Map, src: map::Pos, dst: map::Pos) -> Option<Vec<map::Pos>> {
    let mut min_to_pos = HashMap::new();
    min_to_pos.insert(src, 1);

    let mut q: BinaryHeap<Path> = BinaryHeap::new();
    q.push(Path { path: vec![src] });

    while let Some(wrapper) = q.pop() {
        let path = wrapper.path;

        let cur = *path.last().unwrap();
        if cur == dst {
            return Some(path);
        }

        // If we're on a portal tile, the use the neighbors of the corresponding point.
        //
        // Only include neighbors that:
        // - are actually on the map
        // - don't have a better path leading to them
        let mut neighbors = vec![
            (cur.0 - 1, cur.1),
            (cur.0 + 1, cur.1),
            (cur.0, cur.1 - 1),
            (cur.0, cur.1 + 1),
        ];

        if let Some(other) = map.portal_destinations.get(&cur) {
            neighbors.push(*other);
        }

        for n in neighbors.iter() {
            if !map.available.contains(n) {
                continue;
            }

            let mut new_path = path.to_vec();
            new_path.push(*n);

            if let Some(prev_steps) = min_to_pos.get(n) {
                if *prev_steps <= new_path.len() {
                    continue;
                }
            }

            min_to_pos.insert(*n, new_path.len());
            q.push(Path { path: new_path });
        }
    }

    None
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.to_string()
}

fn main() {
    let m: map::Map = get_input().parse().unwrap();
    println!("{:?}", m);
    let path = shortest_path(&m, m.start, m.end).unwrap();
    println!("{} steps: {:?}", path.len() - 1, path);
}
