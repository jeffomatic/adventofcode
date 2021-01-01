use intcode;
use std::cmp;
use std::collections::{BinaryHeap, HashSet};
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

#[derive(Debug)]
struct Map {
    open: HashSet<(i64, i64)>,
    goal: Option<(i64, i64)>,
}

impl Map {
    fn extrema(&self) -> ((i64, i64), (i64, i64)) {
        let mut min = (i64::max_value(), i64::max_value());
        let mut max = (i64::min_value(), i64::min_value());
        for p in self.open.iter() {
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
        self.open.insert(pos);
        let mut next = Vec::new();

        for dir in dirs().iter() {
            let dir = *dir;
            let next_pos = move_dir(pos, dir);
            if self.open.contains(&next_pos) {
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
                    self.open.insert(next_pos);
                    next.push((next_cpu, next_pos));
                }
                2 => {
                    self.open.insert(next_pos);
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
            open: HashSet::new(),
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
            let pos = *prev_path.last().unwrap();
            if pos == end {
                return prev_path;
            }

            for d in dirs().iter() {
                let next_pos = move_dir(pos, *d);
                if visited.contains(&next_pos) || !self.open.contains(&next_pos) {
                    continue;
                }

                let mut path = prev_path.clone();
                path.push(next_pos);
                q.push(PathSearchNode::from(path));
            }

            visited.insert(pos);
        }

        panic!("could not find path from {:?} to {:?}", start, end)
    }

    fn max_path(&self, start: (i64, i64)) -> Vec<(i64, i64)> {
        let mut max = Vec::new();
        for p in self.open.iter() {
            let path = self.shortest_path(start, *p);
            if path.len() > max.len() {
                max = path;
            }
        }
        max
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
                } else if self.open.contains(&p) {
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

    let start = map.goal.unwrap();
    println!(
        "max path length from {:?}: {}",
        start,
        map.max_path(start).len() - 1, // don't include origin
    );
}
