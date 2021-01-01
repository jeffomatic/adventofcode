use intcode;
use std::cmp;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;
use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
enum Dir {
    N = 1,
    S,
    W,
    E,
}

fn dirs() -> [Dir; 4] {
    return [Dir::N, Dir::S, Dir::W, Dir::E];
}

fn move_dir(pos: (i64, i64), dir: Dir) -> (i64, i64) {
    match dir {
        Dir::N => (pos.0, pos.1 - 1),
        Dir::S => (pos.0, pos.1 + 1),
        Dir::W => (pos.0 - 1, pos.1),
        Dir::E => (pos.0 + 1, pos.1),
    }
}

#[derive(Debug)]
struct Map {
    grid: HashMap<(i64, i64), Vec<Dir>>,
    goal: Option<(i64, i64)>,
}

struct PathSearchNode {
    path: Vec<(i64, i64)>,
}

impl PathSearchNode {
    fn from(path: Vec<(i64, i64)>) -> PathSearchNode {
        PathSearchNode { path: path }
    }
}

impl PartialEq for PathSearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.path.len() == other.path.len()
    }
}

impl Eq for PathSearchNode {}

impl Ord for PathSearchNode {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // Use reverse() because we're using BinaryHeap, which is a maxheap
        self.path.len().cmp(&other.path.len()).reverse()
    }
}

impl PartialOrd for PathSearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Map {
    fn extrema(&self) -> ((i64, i64), (i64, i64)) {
        let mut min = (i64::max_value(), i64::max_value());
        let mut max = (i64::min_value(), i64::min_value());
        for p in self.grid.keys() {
            if p.0 < min.0 {
                min = (p.0, min.1);
            }
            if p.1 < min.1 {
                min = (min.0, p.1);
            }
            if max.0 < p.0 {
                max = (p.0, max.1);
            }
            if max.1 < p.1 {
                max = (max.0, p.1);
            }
        }

        (min, max)
    }

    fn scan(&mut self, cpu: &intcode::Computer, pos: (i64, i64)) {
        self.grid.insert(pos, Vec::new());
        let mut next = Vec::new();

        for dir in dirs().iter() {
            let dir = *dir;
            let next_pos = move_dir(pos, dir);
            if self.grid.contains_key(&next_pos) {
                continue;
            }

            let mut next_cpu = cpu.clone();
            let result = next_cpu.run(&vec![dir as i64]);

            match result.state {
                intcode::State::BlockedOnRead => (),
                intcode::State::Halted => panic!("unexpected halt"),
            }

            if result.output.len() != 1 {
                panic!("unexpected output length: {}", result.output.len());
            }

            match result.output[0] {
                0 => (),
                1 => {
                    self.grid.get_mut(&pos).unwrap().push(dir);
                    next.push((next_cpu, next_pos));
                }
                2 => {
                    self.grid.get_mut(&pos).unwrap().push(dir);
                    next.push((next_cpu, next_pos));
                    self.goal = Some(next_pos);
                }
                unknown => panic!("unknown result code {}", unknown),
            }
        }

        for (next_cpu, next_pos) in next.iter() {
            self.scan(next_cpu, *next_pos);
        }
    }

    fn generate(program: &Vec<i64>) -> Map {
        let cpu = intcode::Computer::new(program);
        let mut map = Map {
            grid: HashMap::new(),
            goal: None,
        };
        map.scan(&cpu, (0, 0));
        map
    }

    fn shortest_path(&self, start: (i64, i64), end: (i64, i64)) -> Vec<(i64, i64)> {
        let mut q = BinaryHeap::from(vec![PathSearchNode::from(vec![start])]);
        let mut visited = HashSet::new();

        while !q.is_empty() {
            let prev_path = q.pop().unwrap().path;
            let prev_pos = *prev_path.last().unwrap();
            if prev_pos == end {
                return prev_path;
            }

            visited.insert(prev_pos);

            let dirs = self.grid.get(&prev_pos).unwrap();
            for p in dirs.iter().map(|d| move_dir(prev_pos, *d)) {
                if visited.contains(&p) {
                    continue;
                }

                let mut path = prev_path.clone();
                path.push(p);
                q.push(PathSearchNode::from(path));
            }
        }

        panic!("could not find path from {:?} to {:?}", start, end)
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (min, max) = self.extrema();
        for j in (min.1 - 1)..=(max.1 + 1) {
            for i in (min.0 - 1)..=(max.0 + 1) {
                let p = (i, j);
                if p == (0, 0) {
                    write!(f, "S")?;
                } else if p == self.goal.unwrap() {
                    write!(f, "X")?;
                } else if self.grid.contains_key(&p) {
                    write!(f, ".")?;
                } else {
                    write!(f, "#")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let program: Vec<i64> = get_input().split(",").map(|s| s.parse().unwrap()).collect();
    let map = Map::generate(&program);
    println!("{}", map);

    let start = (0, 0);
    let path = map.shortest_path(start, map.goal.unwrap());
    println!(
        "path from {:?} to {:?}: {} steps",
        start,
        map.goal.unwrap(),
        path.len() - 1, // don't include origin
    );
}
