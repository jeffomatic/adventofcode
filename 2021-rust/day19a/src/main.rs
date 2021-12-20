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

    fn transform(self, xform: Xform) -> Self {
        self.rot(xform.axis_rotations) + xform.translate
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Xform {
    axis_rotations: Vec3,
    translate: Vec3,
}

fn transform_points(src: &HashSet<Vec3>, xform: Xform) -> HashSet<Vec3> {
    src.iter().map(|p| p.transform(xform)).collect()
}

fn rotate_points(src: &HashSet<Vec3>, rot: Vec3) -> HashSet<Vec3> {
    src.iter().map(|p| p.rot(rot)).collect()
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
    for xrot in 0..=3 {
        for yrot in 0..=3 {
            for zrot in 0..=3 {
                res.push(Vec3 {
                    x: xrot,
                    y: yrot,
                    z: zrot,
                });
            }
        }
    }
    res
}

fn main() {
    let input = get_input();
    let mut lines = input.lines().peekable();
    let mut scanners = Vec::new();

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

        scanners.push(points);
    }

    let min_overlap = 12;

    let mut xforms = HashMap::new();
    xforms.insert(
        0,
        Xform {
            axis_rotations: Vec3::zero(),
            translate: Vec3::zero(),
        },
    );

    let mut unknowns = HashSet::new();
    for i in 1..scanners.len() {
        unknowns.insert(i);
    }

    let mut denylist = HashSet::new();

    'outer: while unknowns.len() > 0 {
        for (a_id, xform) in xforms.clone() {
            let a_points = transform_points(&scanners[a_id], xform);

            for b_id in unknowns.clone() {
                if denylist.contains(&(a_id, b_id)) {
                    continue;
                }

                for axis_rotations in all_rotations() {
                    let b_points = rotate_points(&scanners[b_id], axis_rotations);

                    if let Some(b_to_a) = find_offset(&a_points, &b_points, min_overlap) {
                        // a_origin and b_origin are the same point
                        unknowns.remove(&b_id);
                        xforms.insert(
                            b_id,
                            Xform {
                                axis_rotations,
                                translate: b_to_a,
                            },
                        );
                        continue 'outer;
                    }
                }

                // scanner_id and unknown_id don't overlap; add to denylist so
                // we don't try again.
                denylist.insert((a_id, b_id));
            }
        }

        unreachable!();
    }

    let mut beacons = HashSet::new();
    for (scanner_id, xform) in xforms {
        for p in scanners[scanner_id].iter() {
            beacons.insert(p.transform(xform));
        }
    }

    println!("{}", beacons.len());
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
