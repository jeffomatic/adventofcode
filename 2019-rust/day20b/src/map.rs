use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub type Pos = (usize, usize);

enum PortalDir {
    Up,
    Down,
}

struct Portal {
    pub pos: Pos,
    pub dir: PortalDir,
}

#[derive(Copy, Clone, Debug)]
pub struct Jump {
    pub pos: Pos,
    pub depth_change: i64,
}

#[derive(Debug)]
pub struct Map {
    pub available: HashSet<Pos>,
    pub start: Pos,
    pub end: Pos,
    pub jumps_by_pos: HashMap<Pos, Jump>,
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

    let mut s = 0;
    for y in n..raw.len() {
        let c = raw[y][w];
        if c != '#' && c != '.' {
            s = y - 1;
            break;
        }
    }

    let mut e = 0;
    for x in (w..raw[n].len()).rev() {
        let c = raw[n][x];
        if c == '#' || c == '.' {
            e = x;
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

    // find north/west edges
    let mut n = 0;
    let mut w = 0;
    'nw_interior: for y in exterior.n..exterior.s {
        for x in exterior.w..exterior.e {
            if raw[y][x] == ' ' {
                n = y;
                w = x;
                break 'nw_interior;
            }
        }
    }

    // find south edge
    let mut s = 0;
    for y in n..exterior.s {
        if raw[y][w] == '#' {
            s = y - 1;
            break;
        }
    }

    // find east edge
    let mut e = 0;
    for x in w..exterior.e {
        if raw[n][x] == '#' {
            e = x - 1;
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

// Assume all labels have two characters
fn portal_labels(raw: &RawMap) -> Vec<(String, Portal)> {
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
        res.push((
            label,
            Portal {
                pos: p,
                dir: PortalDir::Up,
            },
        ));
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
        res.push((
            label,
            Portal {
                pos: p,
                dir: PortalDir::Down,
            },
        ));
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
        res.push((
            label,
            Portal {
                pos: p,
                dir: PortalDir::Up,
            },
        ));
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
        res.push((
            label,
            Portal {
                pos: p,
                dir: PortalDir::Down,
            },
        ));
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
        res.push((
            label,
            Portal {
                pos: p,
                dir: PortalDir::Up,
            },
        ));
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
        res.push((
            label,
            Portal {
                pos: p,
                dir: PortalDir::Down,
            },
        ));
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
        res.push((
            label,
            Portal {
                pos: p,
                dir: PortalDir::Up,
            },
        ));
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
        res.push((
            label,
            Portal {
                pos: p,
                dir: PortalDir::Down,
            },
        ));
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

        let mut portals_by_label = HashMap::new();
        for (label, portal) in portal_labels(&raw) {
            match portals_by_label.get_mut(&label) {
                None => {
                    portals_by_label.insert(label, vec![portal]);
                }
                Some(points) => points.push(portal),
            }
        }

        let mut jumps_by_pos = HashMap::new();
        for (_label, portals) in &portals_by_label {
            if portals.len() != 2 {
                continue;
            }

            jumps_by_pos.insert(
                portals[0].pos,
                Jump {
                    pos: portals[1].pos,
                    depth_change: match portals[0].dir {
                        PortalDir::Up => -1,
                        PortalDir::Down => 1,
                    },
                },
            );

            jumps_by_pos.insert(
                portals[1].pos,
                Jump {
                    pos: portals[0].pos,
                    depth_change: match portals[1].dir {
                        PortalDir::Up => -1,
                        PortalDir::Down => 1,
                    },
                },
            );
        }

        Ok(Map {
            available: available_points(&raw),
            start: portals_by_label.get("AA").unwrap()[0].pos,
            end: portals_by_label.get("ZZ").unwrap()[0].pos,
            jumps_by_pos: jumps_by_pos,
        })
    }
}
