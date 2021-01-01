use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, Read};

mod map;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct LayerPos {
    pos: map::Pos,
    depth: usize,
}

#[derive(Clone, Eq, PartialEq)]
struct Path {
    path: Vec<LayerPos>,
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

fn shortest_path(map: &map::Map, src: LayerPos, dst: LayerPos) -> Option<Vec<LayerPos>> {
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

        let mut neighbors = vec![
            LayerPos {
                pos: (cur.pos.0 - 1, cur.pos.1),
                depth: cur.depth,
            },
            LayerPos {
                pos: (cur.pos.0 + 1, cur.pos.1),
                depth: cur.depth,
            },
            LayerPos {
                pos: (cur.pos.0, cur.pos.1 - 1),
                depth: cur.depth,
            },
            LayerPos {
                pos: (cur.pos.0, cur.pos.1 + 1),
                depth: cur.depth,
            },
        ];

        // If we're on a portal tile, include corresponding portal tile.
        if let Some(jump) = map.jumps_by_pos.get(&cur.pos) {
            let new_depth = (cur.depth as i64) + jump.depth_change;
            if new_depth >= 0 {
                neighbors.push(LayerPos {
                    pos: jump.pos,
                    depth: new_depth as usize,
                });
            }
        }

        for n in neighbors.iter() {
            if !map.available.contains(&n.pos) {
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
    let path = shortest_path(
        &m,
        LayerPos {
            pos: m.start,
            depth: 0,
        },
        LayerPos {
            pos: m.end,
            depth: 0,
        },
    )
    .unwrap();
    println!("{} steps", path.len() - 1);
}
