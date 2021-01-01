use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub type Pos = (usize, usize);

#[derive(Debug)]
pub struct Map {
    pub available: HashSet<Pos>,
    pub start: Pos,
    pub end: Pos,
    pub portal_destinations: HashMap<Pos, Pos>,
}

#[derive(Debug)]
struct Extents {
    n: usize,
    s: usize,
    w: usize,
    e: usize,
}

type RawMap = Vec<Vec<char>>;

fn exterior_extents(raw: &RawMap) -> Extents {
    let n = 2;
    let w = 2;

    let mut e = 0;
    for x in (w..raw[n].len()).rev() {
        let c = raw[n][x];
        if c == '#' || c == '.' {
            e = x;
            break;
        }
    }

    let mut s = 0;
    for y in n..raw.len() {
        let c = raw[y][w];
        if c != '#' && c != '.' {
            s = y - 1;
            break;
        }
    }

    Extents {
        n: n,
        s: s,
        w: w,
        e: e,
    }
}

fn interior_extents(raw: &RawMap) -> Extents {
    let exterior = exterior_extents(raw);

    let mut interior = Extents {
        n: 0,
        s: 0,
        w: 0,
        e: 0,
    };

    // find north/west edges
    'nw_interior: for y in exterior.n..exterior.s {
        for x in exterior.w..exterior.e {
            if raw[y][x] == ' ' {
                interior.n = y;
                interior.w = x;
                break 'nw_interior;
            }
        }
    }

    // find east edge
    for x in interior.w..exterior.e {
        if raw[interior.n][x] == '#' {
            interior.e = x - 1;
            break;
        }
    }

    // find south edge
    for y in interior.n..exterior.s {
        if raw[y][interior.w] == '#' {
            interior.s = y - 1;
            break;
        }
    }

    interior
}

// Assume all labels have two characters
fn portal_labels(raw: &RawMap) -> Vec<(String, Pos)> {
    let mut res = Vec::new();

    let exterior = exterior_extents(&raw);
    let interior = interior_extents(&raw);

    // North exterior labels
    for x in exterior.w..=exterior.e {
        let y = 0;
        let c = raw[y][x];
        if c == ' ' {
            continue;
        }

        let label: String = vec![c, raw[y + 1][x]].iter().collect();
        let p = (x, exterior.n);
        res.push((label, p));
    }

    // North interior labels
    for x in interior.w..=interior.e {
        let y = interior.n;
        let c = raw[y][x];
        if c == ' ' {
            continue;
        }

        let label: String = vec![c, raw[y + 1][x]].iter().collect();
        let p = (x, interior.n - 1);
        res.push((label, p));
    }

    // South exterior labels
    for x in exterior.w..=exterior.e {
        let y = exterior.s + 1;
        let c = raw[y][x];
        if c == ' ' {
            continue;
        }

        let label: String = vec![c, raw[y + 1][x]].iter().collect();
        let p = (x, exterior.s);
        res.push((label, p));
    }

    // South interior labels
    for x in interior.w..=interior.e {
        let y = interior.s - 1;
        let c = raw[y][x];
        if c == ' ' {
            continue;
        }

        let label: String = vec![c, raw[y + 1][x]].iter().collect();
        let p = (x, interior.s + 1);
        res.push((label, p));
    }

    // West exterior labels
    for y in exterior.n..=exterior.s {
        let x = exterior.w - 2;
        let c = raw[y][x];
        if c == ' ' {
            continue;
        }

        let label: String = vec![c, raw[y][x + 1]].iter().collect();
        let p = (exterior.w, y);
        res.push((label, p));
    }

    // West interior labels
    for y in interior.n..=interior.s {
        let x = interior.w;
        let c = raw[y][x];
        if c == ' ' {
            continue;
        }

        let label: String = vec![c, raw[y][x + 1]].iter().collect();
        let p = (interior.w - 1, y);
        res.push((label, p));
    }

    // East exterior labels
    for y in exterior.n..=exterior.s {
        let x = exterior.e + 1;
        if x >= raw[y].len() {
            continue; // compensate for first example, which has no eastern labels
        }

        let c = raw[y][x];
        if c == ' ' {
            continue;
        }

        let label: String = vec![c, raw[y][x + 1]].iter().collect();
        let p = (exterior.e, y);
        res.push((label, p));
    }

    // East interior labels
    for y in interior.n..=interior.s {
        let x = interior.e - 1;
        let c = raw[y][x];
        if c == ' ' {
            continue;
        }

        let label: String = vec![c, raw[y][x + 1]].iter().collect();
        let p = (interior.e + 1, y);
        res.push((label, p));
    }

    res
}

fn available_points(raw: &RawMap) -> HashSet<Pos> {
    let mut res = HashSet::new();

    let exterior = exterior_extents(&raw);
    let interior = interior_extents(&raw);

    for y in exterior.n..=exterior.s {
        for x in exterior.w..=exterior.e {
            // skip interior of frame
            if interior.n <= y && y <= interior.s && interior.w <= x && x <= interior.e {
                continue;
            }

            if raw[y][x] == '.' {
                res.insert((x, y));
            }
        }
    }

    res
}

impl FromStr for Map {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut raw: RawMap = s
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        // Make sure all rows are the same length
        let width = raw.iter().fold(0, |acc, row| std::cmp::max(acc, row.len()));
        for row in raw.iter_mut() {
            for _ in row.len()..width {
                row.push(' ');
            }
        }

        let mut pos_by_label = HashMap::new();
        for (label, p) in portal_labels(&raw) {
            match pos_by_label.get_mut(&label) {
                None => {
                    pos_by_label.insert(label, vec![p]);
                }
                Some(points) => points.push(p),
            }
        }

        let mut portal_destinations = HashMap::new();
        for (label, points) in &pos_by_label {
            if points.len() == 2 {
                portal_destinations.insert(points[0], points[1]);
                portal_destinations.insert(points[1], points[0]);
            }
        }

        Ok(Map {
            available: available_points(&raw),
            start: pos_by_label.get("AA").unwrap()[0],
            end: pos_by_label.get("ZZ").unwrap()[0],
            portal_destinations: portal_destinations,
        })
    }
}
