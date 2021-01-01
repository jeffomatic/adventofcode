use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::{self, Read};
use std::str::FromStr;

type Point = (usize, usize);

#[derive(Clone, Eq, PartialEq)]
struct Path {
    path: Vec<Point>,
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

fn shortest_path(reachable: &HashSet<Point>, src: Point, dst: Point) -> Option<Vec<Point>> {
    let mut visited = HashSet::new();
    visited.insert(src);

    let mut q: BinaryHeap<Path> = BinaryHeap::new();
    q.push(Path { path: vec![src] });

    while let Some(wrapper) = q.pop() {
        let path = wrapper.path;
        let cur = *path.last().unwrap();

        if cur == dst {
            return Some(path);
        }

        for n in [
            (cur.0 - 1, cur.1),
            (cur.0 + 1, cur.1),
            (cur.0, cur.1 - 1),
            (cur.0, cur.1 + 1),
        ]
        .iter()
        {
            if !reachable.contains(n) || visited.contains(n) {
                continue;
            }

            let mut new_path = path.to_vec();
            new_path.push(*n);
            q.push(Path { path: new_path });

            visited.insert(*n);
        }
    }

    None
}

#[derive(Debug)]
struct MapEdge {
    src: Point,
    dst: Point,
    steps: usize,
    doors: HashSet<char>,
    keys: Vec<char>,
}

#[derive(Debug)]
struct Map {
    open: HashSet<Point>,
    starting_points: HashSet<Point>,
    doors: HashMap<Point, char>,
    keys: HashMap<Point, char>,
    graph: HashMap<(Point, Point), MapEdge>,
}

impl Map {
    fn make_edge(&self, src: Point, dst: Point) -> Option<MapEdge> {
        let mut points = shortest_path(&self.open, src, dst)?;

        // ignore first item, which is the source.
        points.remove(0);

        // Walk the points and collect key/door information
        let mut doors = HashSet::new();
        let mut keys = Vec::new();

        for p in points.iter() {
            if let Some(id) = self.doors.get(p) {
                doors.insert(*id);
            }

            if let Some(id) = self.keys.get(p) {
                keys.push(*id);
            }
        }

        Some(MapEdge {
            src: src,
            dst: dst,
            steps: points.len(),
            doors: doors,
            keys: keys,
        })
    }

    fn populate_graph(&mut self) {
        let key_locs: Vec<Point> = self.keys.keys().cloned().collect();

        // Add a path between the starting point and each key location.
        for keypos in key_locs.iter() {
            for start_pos in self.starting_points.iter() {
                if let Some(edge) = self.make_edge(*start_pos, *keypos) {
                    self.graph.insert((*start_pos, *keypos), edge);
                }
            }
        }

        // Add a path between each key and another key.
        for i in 0..key_locs.len() - 1 {
            for j in (i + 1)..key_locs.len() {
                let p1 = key_locs[i];
                let p2 = key_locs[j];

                if let Some(edge) = self.make_edge(p1, p2) {
                    self.graph.insert((p1, p2), edge);
                }

                if let Some(edge) = self.make_edge(p2, p1) {
                    self.graph.insert((p2, p1), edge);
                }
            }
        }
    }
}

impl FromStr for Map {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Map {
            open: HashSet::new(),
            starting_points: HashSet::new(),
            doors: HashMap::new(),
            keys: HashMap::new(),
            graph: HashMap::new(),
        };

        let lines: Vec<&str> = s.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                let p = (j, i);

                if c != '#' {
                    map.open.insert(p);
                }

                if c == '@' {
                    map.starting_points.insert(p);
                } else if (b'A'..=b'Z').contains(&(c as u8)) {
                    map.doors.insert(p, c.to_ascii_lowercase());
                } else if (b'a'..=b'z').contains(&(c as u8)) {
                    map.keys.insert(p, c);
                }
            }
        }

        map.populate_graph();

        return Ok(map);
    }
}

#[derive(Clone, Debug)]
struct State {
    positions: HashSet<Point>,
    steps: usize,
    keys: Vec<char>,
}

impl State {
    fn new(starting_points: &HashSet<Point>) -> State {
        State {
            positions: starting_points.iter().map(|p| *p).collect(),
            steps: 0,
            keys: Vec::new(),
        }
    }

    fn edges_to_next_keys<'a>(&self, map: &'a Map) -> Vec<&'a MapEdge> {
        let mut edges = Vec::new();

        for (keypos, key) in map.keys.iter() {
            // Don't worry about keys we already have.
            if self.keys.contains(key) {
                continue;
            }

            for p in self.positions.iter() {
                if let Some(edge) = map.graph.get(&(*p, *keypos)) {
                    // We can't use the edge if we don't have the right keys.
                    let keyset: HashSet<char> = self.keys.iter().map(|c| *c).collect();
                    if !edge.doors.is_subset(&keyset) {
                        continue;
                    }

                    edges.push(edge);
                }
            }
        }

        edges
    }

    fn follow_edge(&mut self, edge: &MapEdge) {
        self.positions.remove(&edge.src);
        self.positions.insert(edge.dst);
        self.steps += edge.steps;

        // only add new keys
        for k in edge.keys.iter() {
            if !self.keys.contains(k) {
                self.keys.push(*k)
            }
        }
    }

    fn search_key(&self) -> (Vec<Point>, String) {
        let mut positions: Vec<Point> = self.positions.iter().cloned().collect();
        positions.sort();
        let mut keys: Vec<String> = self.keys.iter().map(|c| c.to_string()).collect();
        keys.sort();
        (positions, keys.join(""))
    }
}

struct StateForSearch {
    state: State,
}

impl PartialEq for StateForSearch {
    fn eq(&self, other: &Self) -> bool {
        self.state.steps == other.state.steps
    }
}

impl Eq for StateForSearch {}

impl Ord for StateForSearch {
    // Ordering for maxheap:
    // - lowest number of steps should always go to the top
    // - equal number of steps should prioritize state with larger number of keys
    fn cmp(&self, other: &Self) -> Ordering {
        match other.state.steps.cmp(&self.state.steps) {
            Ordering::Equal => self.state.keys.len().cmp(&other.state.keys.len()),
            ord => ord,
        }
    }
}

impl PartialOrd for StateForSearch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn search(map: &Map) -> Option<State> {
    let s = State::new(&map.starting_points);

    let mut visited = HashMap::new();
    let mut q: BinaryHeap<StateForSearch> = BinaryHeap::new();
    q.push(StateForSearch { state: s });

    while let Some(wrapper) = q.pop() {
        let state = wrapper.state;

        if state.keys.len() == map.keys.len() {
            return Some(state);
        }

        for edge in state.edges_to_next_keys(&map).iter() {
            let mut new_state = state.clone();
            new_state.follow_edge(edge);

            // don't continue if we've reached a point where a better path
            // already exists
            let search_key = new_state.search_key();
            if let Some(steps) = visited.get(&search_key) {
                if new_state.steps >= *steps {
                    continue;
                }
            }

            visited.insert(search_key, new_state.steps);
            q.push(StateForSearch { state: new_state });
        }
    }

    None
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let map: Map = get_input().parse().unwrap();
    println!("{:?}", search(&map).unwrap());
}
