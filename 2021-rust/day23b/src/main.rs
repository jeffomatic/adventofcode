use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Type {
    Amber,
    Bronze,
    Copper,
    Desert,
}

const ROOM_HEIGHT: i32 = 4;

impl Type {
    // must be sorted from greatest depth to least depth
    fn homeroom_positions(&self) -> Vec<(i32, i32)> {
        match self {
            Type::Amber => (0..ROOM_HEIGHT).rev().map(|n| (2 + n, 3)).collect(),
            Type::Bronze => (0..ROOM_HEIGHT).rev().map(|n| (2 + n, 5)).collect(),
            Type::Copper => (0..ROOM_HEIGHT).rev().map(|n| (2 + n, 7)).collect(),
            Type::Desert => (0..ROOM_HEIGHT).rev().map(|n| (2 + n, 9)).collect(),
        }
    }

    fn step_cost(&self) -> i32 {
        match self {
            Type::Amber => 1,
            Type::Bronze => 10,
            Type::Copper => 100,
            Type::Desert => 1000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Amphipod {
    typ: Type,
    // row, col
    pos: (i32, i32),
}

fn all_hallway_spaces() -> Vec<(i32, i32)> {
    vec![
        // hallway spaces
        (1, 1),
        (1, 2),
        // (1, 3),
        (1, 4),
        // (1, 5),
        (1, 6),
        // (1, 7),
        (1, 8),
        // (1, 9),
        (1, 10),
        (1, 11),
    ]
}

fn is_hallway_space(pos: (i32, i32)) -> bool {
    pos.0 == 1 && 1 <= pos.1 && pos.1 <= 11
}

fn has_path_vert(
    src_row: i32,
    dst_row: i32,
    col: i32,
    occupied: &HashMap<(i32, i32), Type>,
) -> bool {
    let mut row = src_row;
    let inc = (dst_row - src_row).signum();
    while row != dst_row {
        row += inc;
        if occupied.contains_key(&(row, col)) {
            return false;
        }
    }
    return true;
}

fn has_path_horz(
    src_col: i32,
    dst_col: i32,
    row: i32,
    occupied: &HashMap<(i32, i32), Type>,
) -> bool {
    let mut col = src_col;
    let inc = (dst_col - src_col).signum();
    while col != dst_col {
        col += inc;
        if occupied.contains_key(&(row, col)) {
            return false;
        }
    }
    return true;
}

fn get_distance(src: (i32, i32), dst: (i32, i32)) -> i32 {
    // room-to-room path
    if src.0 > 1 && dst.0 > 1 {
        return manhattan(src, (1, src.1)) + manhattan((1, src.1), dst);
    }

    manhattan(src, dst)
}

fn has_path(src: (i32, i32), dst: (i32, i32), occupied: &HashMap<(i32, i32), Type>) -> bool {
    if src == dst {
        return true;
    }

    // room-to-room path
    if src.0 > 1 && dst.0 > 1 {
        return has_path_vert(src.0, 1, src.1, occupied)
            && has_path_horz(src.1, dst.1, 1, occupied)
            && has_path_vert(1, dst.0, dst.1, occupied);
    }

    // hallway-to-hallway path
    if src.0 == 1 && dst.0 == 1 {
        return has_path_horz(src.1, dst.1, 1, occupied);
    }

    // hallway-to-room path
    if src.0 == 1 {
        return has_path_horz(src.1, dst.1, 1, occupied)
            && has_path_vert(1, dst.0, dst.1, occupied);
    }

    // room-to-hallway path
    has_path_vert(src.0, 1, src.1, occupied) && has_path_horz(src.1, dst.1, 1, occupied)
}

fn get_intended_position(apod: &Amphipod, map: &HashMap<(i32, i32), Type>) -> (i32, i32) {
    for dst in apod.typ.homeroom_positions() {
        if dst == apod.pos {
            return dst;
        }

        match map.get(&dst) {
            None => return dst,
            Some(&other) => {
                if other != apod.typ {
                    return dst;
                }
            }
        }
    }

    unreachable!()
}

fn get_next_moves(src: &Vec<Amphipod>) -> Vec<(usize, (i32, i32))> {
    let mut res = Vec::new();

    let map = src.iter().fold(HashMap::new(), |mut accum, a| {
        accum.insert(a.pos, a.typ);
        accum
    });

    for (i, apod) in src.iter().enumerate() {
        // println!("getting next moves for {:?}", apod);
        let intended_pos = get_intended_position(&apod, &map);
        if intended_pos == apod.pos {
            continue;
        }

        if has_path(apod.pos, intended_pos, &map) {
            res.push((i, intended_pos));
            continue;
        }

        if is_hallway_space(apod.pos) {
            continue;
        }

        // apod is in the incorrect room, can't move to its intended room, and
        // therefore can only move into the hallway.
        for dst in all_hallway_spaces() {
            if !has_path(apod.pos, dst, &map) {
                continue;
            }
            res.push((i, dst));
        }
    }

    res
}

fn is_finished(state: &Vec<Amphipod>) -> bool {
    state
        .iter()
        .all(|a| a.typ.homeroom_positions().contains(&a.pos))
}

fn manhattan(src: (i32, i32), dst: (i32, i32)) -> i32 {
    (dst.0 - src.0).abs() + (dst.1 - src.1).abs()
}

fn parse(src: &str) -> Vec<Amphipod> {
    let mut res = Vec::new();
    for (i, line) in src.trim().lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                'A' => res.push(Amphipod {
                    typ: Type::Amber,
                    pos: (i as i32, j as i32),
                }),
                'B' => res.push(Amphipod {
                    typ: Type::Bronze,
                    pos: (i as i32, j as i32),
                }),
                'C' => res.push(Amphipod {
                    typ: Type::Copper,
                    pos: (i as i32, j as i32),
                }),
                'D' => res.push(Amphipod {
                    typ: Type::Desert,
                    pos: (i as i32, j as i32),
                }),
                _ => (),
            }
        }
    }

    res.sort();
    res
}

fn best_cost(state: &Vec<Amphipod>, cost: i32, cache: &mut HashMap<Vec<Amphipod>, i32>) -> i32 {
    if is_finished(state) {
        return cost;
    }

    let mut best = i32::MAX;

    for (id, next_pos) in get_next_moves(&state) {
        let apod = state[id];
        let mut next_state = state.clone();
        next_state[id] = Amphipod {
            typ: apod.typ,
            pos: next_pos,
        };
        next_state.sort();

        let next_cost = cost + get_distance(apod.pos, next_pos) * apod.typ.step_cost();
        if let Some(&prev_cost) = cache.get(&next_state) {
            if prev_cost <= next_cost {
                continue;
            }
        }

        cache.insert(next_state.clone(), next_cost);
        best = best.min(best_cost(&next_state, next_cost, cache));
    }

    best
}

fn main() {
    let src = r"
#############
#...........#
###D#C#D#B###
  #D#C#B#A#
  #D#B#A#C#
  #C#A#A#B#
  #########
    ";

    let start = parse(src);
    println!("{}", best_cost(&start, 0, &mut HashMap::new()));
}
