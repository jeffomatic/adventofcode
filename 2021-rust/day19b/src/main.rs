use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
    ops::{Add, Neg, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vec3 {
    fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    fn rot(self, axis_rotations: Self) -> Self {
        let mut res = self;

        // x-axis rotation
        for _ in 0..axis_rotations.x {
            let prev_z = res.z;
            res.z = res.y;
            res.y = -prev_z;
        }

        // y-axis rotation
        for _ in 0..axis_rotations.y {
            let prev_z = res.z;
            res.z = -res.x;
            res.x = prev_z;
        }

        // z-axis rotation
        for _ in 0..axis_rotations.z {
            let prev_y = res.y;
            res.y = res.x;
            res.x = -prev_y;
        }

        res
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Xform {
    axis_rotations: Vec3,
    translate: Vec3,
}

fn translate_points(src: &HashSet<Vec3>, translate: Vec3) -> HashSet<Vec3> {
    src.iter().map(|p| *p + translate).collect()
}

fn find_offset(a: &HashSet<Vec3>, b: &HashSet<Vec3>, min_similar: usize) -> Option<Vec3> {
    for a_origin in a.iter() {
        let a_translated = translate_points(a, -*a_origin);
        for b_origin in b.iter() {
            let b_translated = translate_points(b, -*b_origin);
            if a_translated.intersection(&b_translated).count() >= min_similar {
                return Some(*a_origin - *b_origin);
            }
        }
    }

    None
}

fn all_rotations() -> Vec<Vec3> {
    let mut res = Vec::new();
    let mut known_rots = HashSet::new();

    for xrot in 0..=3 {
        for yrot in 0..=3 {
            for zrot in 0..=3 {
                let rot = Vec3 {
                    x: xrot,
                    y: yrot,
                    z: zrot,
                };
                let axes_rotated = (
                    Vec3 { x: 1, y: 0, z: 0 }.rot(rot),
                    Vec3 { x: 0, y: 1, z: 0 }.rot(rot),
                    Vec3 { x: 0, y: 0, z: 1 }.rot(rot),
                );
                if known_rots.contains(&axes_rotated) {
                    continue;
                }

                known_rots.insert(axes_rotated);
                res.push(rot);
            }
        }
    }

    res
}

fn main() {
    let input = get_input();
    let mut lines = input.lines().peekable();
    let mut scanners = Vec::new();
    let all_rots = all_rotations();

    while lines.peek().is_some() {
        // consume hyphen
        lines.next();

        let mut points = HashSet::new();
        loop {
            if lines.peek().is_none() {
                break;
            }

            let line = lines.next().unwrap();
            if line.len() == 0 {
                break;
            }

            let coords: Vec<i32> = line.split(",").map(|toks| toks.parse().unwrap()).collect();
            points.insert(Vec3 {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            });
        }

        // Insert the scanner's list of beacon positions, along with a cache of
        // pre-rotated positions. This allows us to avoid re-processing
        // rotations during brute-force search.
        let mut rots: HashMap<Vec3, HashSet<Vec3>> = HashMap::new();
        for r in all_rots.iter() {
            rots.insert(*r, points.iter().map(|p| p.rot(*r)).collect());
        }

        scanners.push(rots);
    }

    let min_overlap = 12;

    // `resolved` is a map between scanner IDs and a tuple of the scanner's
    // beacon positions, transformed to scanner 0's space.
    let mut resolved = HashMap::new();
    resolved.insert(0, scanners[0][&Vec3::zero()].clone());
    let mut offsets = HashMap::new();
    offsets.insert(0, Vec3::zero());

    let mut unresolved = HashSet::new();
    for i in 1..scanners.len() {
        unresolved.insert(i);
    }

    let mut unrelated_scanners = HashSet::new();

    'outer: while unresolved.len() > 0 {
        for a_id in resolved.keys().cloned() {
            let a_points = &resolved[&a_id];

            for b_id in unresolved.clone() {
                if unrelated_scanners.contains(&(a_id, b_id)) {
                    continue;
                }

                for rot in all_rots.iter() {
                    let b_points = &scanners[b_id][rot];
                    if let Some(b_to_a) = find_offset(a_points, b_points, min_overlap) {
                        unresolved.remove(&b_id);
                        resolved.insert(b_id, translate_points(b_points, b_to_a));
                        offsets.insert(b_id, b_to_a);
                        continue 'outer;
                    }
                }

                unrelated_scanners.insert((a_id, b_id));
            }
        }

        unreachable!();
    }

    let offsets: Vec<Vec3> = offsets.values().map(|v| *v).collect();
    let mut best = 0;
    for i in 0..offsets.len() - 1 {
        for j in (i + 1)..offsets.len() {
            let d = offsets[j] - offsets[i];
            best = best.max(d.x.abs() + d.y.abs() + d.z.abs());
        }
    }

    println!("{}", best);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
