use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Variant {
    id: i64,
    n: String,
    s: String,
    w: String,
    e: String,
}

impl Variant {
    fn new(id: i64, image: &Vec<Vec<char>>) -> Variant {
        let dim = image.len();
        Variant {
            id,
            n: image.first().unwrap().iter().collect(),
            s: image.last().unwrap().iter().collect(),
            w: image.iter().map(|line| line[0]).collect(),
            e: image.iter().map(|line| line[dim - 1]).collect(),
        }
    }

    fn rotate(&self) -> Variant {
        Variant {
            id: self.id,
            n: self.w.chars().rev().collect(),
            s: self.e.chars().rev().collect(),
            w: self.s.clone(),
            e: self.n.clone(),
        }
    }

    fn fliph(&self) -> Variant {
        Variant {
            id: self.id,
            n: self.n.chars().rev().collect(),
            s: self.s.chars().rev().collect(),
            w: self.e.clone(),
            e: self.w.clone(),
        }
    }

    fn flipv(&self) -> Variant {
        Variant {
            id: self.id,
            n: self.s.clone(),
            s: self.n.clone(),
            w: self.w.chars().rev().collect(),
            e: self.e.chars().rev().collect(),
        }
    }

    fn variants(&self) -> Vec<Variant> {
        let a = self.clone();
        let b = a.rotate();
        let c = b.rotate();
        let d = c.rotate();
        vec![
            a.clone(),
            a.fliph(), // same as c.flipv()
            a.flipv(), // same as c.fliph()
            b.clone(),
            b.fliph(), // same as d.flipv()
            b.flipv(), // same as d.fliph()
            c,
            d,
        ]
    }
}

#[derive(Debug)]
struct Index {
    n: HashMap<String, HashSet<Variant>>,
    w: HashMap<String, HashSet<Variant>>,
}

impl Index {
    fn new(variants: Vec<Variant>) -> Index {
        let mut index = Index {
            n: HashMap::new(),
            w: HashMap::new(),
        };

        for v in variants {
            index
                .n
                .entry(v.n.clone())
                .or_insert_with(|| HashSet::new())
                .insert(v.clone());
            index
                .w
                .entry(v.w.clone())
                .or_insert_with(|| HashSet::new())
                .insert(v);
        }

        index
    }
}

#[derive(Debug, Clone)]
struct Composite {
    dim: usize,
    items: Vec<Variant>,
}

impl Composite {
    fn new(dim: usize, v: Variant) -> Composite {
        Composite {
            dim,
            items: vec![v],
        }
    }

    fn is_complete(&self) -> bool {
        self.items.len() == self.dim * self.dim
    }

    fn score(&self) -> i64 {
        self.items[0].id
            * self.items[self.dim - 1].id
            * self.items[self.dim * (self.dim - 1)].id
            * self.items[self.dim * self.dim - 1].id
    }

    fn eastern_candidates(&self, n: usize, index: &Index) -> HashSet<Variant> {
        index
            .w
            .get(&self.items[n].e)
            .map_or_else(|| HashSet::new(), |set| set.clone())
    }

    fn southern_candidates(&self, n: usize, index: &Index) -> HashSet<Variant> {
        index
            .n
            .get(&self.items[n].s)
            .map_or_else(|| HashSet::new(), |set| set.clone())
    }

    fn next_candidates(&self, index: &Index) -> Vec<Composite> {
        let candidates: HashSet<Variant> = if self.items.len() < self.dim {
            // First row; only check for eastern matches
            self.eastern_candidates(self.items.len() - 1, index)
        } else if self.items.len() % self.dim == 0 {
            // First item in new row; only check for southern matches
            self.southern_candidates(self.items.len() - self.dim, index)
        } else {
            // Check for both eastern and southern matches
            self.eastern_candidates(self.items.len() - 1, index)
                .intersection(&self.southern_candidates(self.items.len() - self.dim, index))
                .cloned()
                .collect()
        };

        let used: HashSet<i64> = self.items.iter().map(|v| v.id).collect();
        candidates
            .iter()
            .filter(|v| !used.contains(&v.id))
            .map(|v| {
                let mut next = (*self).clone();
                next.items.push(v.clone());
                next
            })
            .collect()
    }
}

fn search(composite: Composite, index: &Index) -> Option<Composite> {
    for c in composite.next_candidates(index) {
        if c.is_complete() {
            return Some(c);
        }

        if let Some(res) = search(c, index) {
            return Some(res);
        }
    }

    None
}

fn main() {
    let tile_id_re = Regex::new(r#"Tile (\d+):"#).unwrap();

    let mut src_tiles: HashMap<i64, Variant> = HashMap::new();
    for chunk in get_input().split("\n\n") {
        let lines: Vec<&str> = chunk.lines().collect();
        let (id_s, image_s) = (lines[0], &lines[1..]);

        let id: i64 = tile_id_re.captures(id_s).unwrap()[1].parse().unwrap();
        let image: Vec<Vec<char>> = image_s.iter().map(|line| line.chars().collect()).collect();
        src_tiles.insert(id, Variant::new(id, &image));
    }

    let dim = (src_tiles.len() as f64).sqrt() as usize;
    let all_variants: Vec<Variant> = src_tiles.iter().flat_map(|(_, v)| v.variants()).collect();
    let index = Index::new(all_variants.clone());

    for v in all_variants.iter() {
        let c = Composite::new(dim, v.clone());
        if let Some(res) = search(c, &index) {
            println!("{}", res.score(),);
            return;
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
