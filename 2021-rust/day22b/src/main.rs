use std::{
    io::{self, Read},
    ops::{Index, IndexMut},
};

fn main() {
    let instructions: Vec<Instruction> = get_input().lines().map(parse_instruction).collect();
    let mut active: Vec<Cuboid> = Vec::new();
    for ins in instructions {
        active = if ins.on {
            add_cuboid(active, &ins.vol)
        } else {
            remove_cuboid(active, &ins.vol)
        };
    }

    let total_vol = active.iter().fold(0, |accum, v| accum + v.volume());
    println!("{}", total_vol);
}

fn add_cuboid(src: Vec<Cuboid>, to_add: &Cuboid) -> Vec<Cuboid> {
    let mut res = remove_cuboid(src, to_add);
    res.push(to_add.clone());
    res
}

fn remove_cuboid(src: Vec<Cuboid>, to_remove: &Cuboid) -> Vec<Cuboid> {
    let mut res = Vec::new();

    for c in src {
        // if to_remove and c are disjoint, just add c to the result
        if !to_remove.overlap(&c) {
            res.push(c);
            continue;
        }

        // if to_remove completely subsumes c, don't add c to res and continue
        if to_remove.encloses(&c) {
            continue;
        }

        // If we get here, c either completely encloses to_remove, or c
        // partially overlaps to_remove. In either case, split c into multiple
        // chunks, removing the overlap between c and to_remove.
        res.append(&mut c.difference(to_remove));
    }

    res
}

#[derive(Debug, Clone)]
struct Cuboid {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl Index<usize> for Cuboid {
    type Output = (i64, i64);

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of range {}", index),
        }
    }
}

impl IndexMut<usize> for Cuboid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of range {}", index),
        }
    }
}

impl Cuboid {
    fn volume(&self) -> i64 {
        (self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)
    }

    fn overlap(&self, other: &Self) -> bool {
        return range_overlap(self.x, other.x)
            && range_overlap(self.y, other.y)
            && range_overlap(self.z, other.z);
    }

    fn encloses(&self, other: &Self) -> bool {
        return range_encloses(self.x, other.x)
            && range_encloses(self.y, other.y)
            && range_encloses(self.z, other.z);
    }

    fn difference(&self, other: &Self) -> Vec<Self> {
        for axis in 0..=2 {
            let split_overlap = split_overlaps(self[axis], other[axis]);
            if let Some(nonoverlap) = split_overlap.a_min_nonoverlap {
                let mut chunk = self.clone();
                chunk[axis] = nonoverlap;

                let mut res = vec![chunk];
                if nonoverlap.1 < self[axis].1 {
                    let mut rest = self.clone();
                    rest[axis] = (nonoverlap.1 + 1, self[axis].1);
                    let mut next_res = rest.difference(other);
                    res.append(&mut next_res);
                }

                return res;
            }

            if let Some(nonoverlap) = split_overlap.a_max_nonoverlap {
                let mut chunk = self.clone();
                chunk[axis] = nonoverlap;

                let mut res = vec![chunk];
                if nonoverlap.0 > self[axis].0 {
                    let mut rest = self.clone();
                    rest[axis] = (self[axis].0, nonoverlap.0 - 1);
                    let mut next_res = rest.difference(other);
                    res.append(&mut next_res);
                }

                return res;
            }
        }

        Vec::new()
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    on: bool,
    vol: Cuboid,
}

fn range_overlap(a: (i64, i64), b: (i64, i64)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

fn range_encloses(a: (i64, i64), b: (i64, i64)) -> bool {
    a.0 <= b.0 && b.1 <= a.1
}

struct RangeSplit {
    overlap: Option<(i64, i64)>,
    a_min_nonoverlap: Option<(i64, i64)>,
    a_max_nonoverlap: Option<(i64, i64)>,
    b_min_nonoverlap: Option<(i64, i64)>,
    b_max_nonoverlap: Option<(i64, i64)>,
}

fn split_overlaps(a: (i64, i64), b: (i64, i64)) -> RangeSplit {
    let mut res = RangeSplit {
        overlap: None,
        a_min_nonoverlap: None,
        a_max_nonoverlap: None,
        b_min_nonoverlap: None,
        b_max_nonoverlap: None,
    };

    if range_overlap(a, b) {
        res.overlap = Some((a.0.max(b.0), a.1.min(b.1)));
    }

    if a.0 < b.0 {
        res.a_min_nonoverlap = Some((a.0, b.0 - 1));
    }

    if a.1 > b.1 {
        res.a_max_nonoverlap = Some((b.1 + 1, a.1));
    }

    if b.0 < a.0 {
        res.b_min_nonoverlap = Some((b.0, a.0 - 1));
    }

    if b.1 > a.1 {
        res.b_max_nonoverlap = Some((a.1 + 1, b.1));
    }

    res
}

// parses expressions of the following form: x=-24..25
fn parse_range(mut src: &str) -> (i64, i64) {
    // ignore `x=`
    src = &src[2..];
    let mut toks = src.split("..");
    (
        toks.next().unwrap().parse().unwrap(),
        toks.next().unwrap().parse().unwrap(),
    )
}

// parses expression of the following form:
//  on x=-24..25,y=-36..8,z=-15..31
//  off x=-39..-20,y=-32..-18,z=36..47
fn parse_instruction(mut line: &str) -> Instruction {
    let on = if line.starts_with("on ") {
        line = &line[3..];
        true
    } else {
        line = &line[4..];
        false
    };

    let mut toks = line.split(",");

    Instruction {
        on,
        vol: Cuboid {
            x: parse_range(toks.next().unwrap()),
            y: parse_range(toks.next().unwrap()),
            z: parse_range(toks.next().unwrap()),
        },
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
