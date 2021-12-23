use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Type {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Type {
    fn homeroom_positions(&self) -> Vec<(i32, i32)> {
        match self {
            Type::Amber => vec![(2, 3), (3, 3)],
            Type::Bronze => vec![(2, 5), (3, 5)],
            Type::Copper => vec![(2, 7), (3, 7)],
            Type::Desert => vec![(2, 9), (3, 9)],
        }
    }

    fn homeroom_is_friendly(&self, state: &HashMap<(i32, i32), Type>) -> bool {
        self.homeroom_positions()
            .iter()
            .all(|pos| match state.get(pos) {
                None => true,
                Some(typ) => typ == self,
            })
    }

    fn step_cost(&self) -> i32 {
        match self {
            Type::Amber => 1,
            Type::Bronze => 10,
            Type::Copper => 100,
            Type::Desert => 1000,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Type::Amber => 'A',
            Type::Bronze => 'B',
            Type::Copper => 'C',
            Type::Desert => 'D',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

fn is_room_space(pos: (i32, i32)) -> bool {
    (pos.0 == 2 || pos.0 == 3) && (pos.1 == 3 || pos.1 == 5 || pos.1 == 7 || pos.1 == 9)
}

fn is_occupiable_space(pos: (i32, i32)) -> bool {
    is_hallway_space(pos) || is_room_space(pos)
}

fn has_path(src: (i32, i32), dst: (i32, i32), occupied: &HashMap<(i32, i32), Type>) -> bool {
    if src == dst {
        return true;
    }

    let di = dst.0 - src.0;
    if di != 0 {
        let next = (src.0 + di.signum(), src.1);
        if is_occupiable_space(next) && !occupied.contains_key(&next) {
            return has_path(next, dst, occupied);
        }
    }

    let dj = dst.1 - src.1;
    if dj != 0 {
        let next = (src.0, src.1 + dj.signum());
        if is_occupiable_space(next) && !occupied.contains_key(&next) {
            return has_path(next, dst, occupied);
        }
    }

    false
}

fn get_next_moves(src: &Vec<Amphipod>) -> Vec<(usize, (i32, i32))> {
    let mut res = Vec::new();

    let state = src.iter().fold(HashMap::new(), |mut accum, a| {
        accum.insert(a.pos, a.typ);
        accum
    });

    for (i, apod) in src.iter().enumerate() {
        if is_hallway_space(apod.pos) {
            if !apod.typ.homeroom_is_friendly(&state) {
                continue;
            }

            for dst in apod.typ.homeroom_positions() {
                if has_path(apod.pos, dst, &state) {
                    res.push((i, dst));
                }
            }

            continue;
        }

        // apod is in a room space
        for dst in all_hallway_spaces() {
            if dst == apod.pos || !has_path(apod.pos, dst, &state) {
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

fn man_dist(src: (i32, i32), dst: (i32, i32)) -> i32 {
    (dst.0 - src.0).abs() + (dst.1 - src.1).abs()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    state: Vec<Amphipod>,
    cost: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // flip order for maxheap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
    res
}

fn apply_move(src: &Vec<Amphipod>, mov: (usize, (i32, i32))) -> Vec<Amphipod> {
    let prev_apod = src[mov.0];
    let mut next_state = src.clone();
    next_state[mov.0] = Amphipod {
        typ: prev_apod.typ,
        pos: mov.1,
    };
    next_state
}

fn print_state(src: &Vec<Amphipod>) {
    let by_pos = src.iter().fold(HashMap::new(), |mut accum, a| {
        accum.insert(a.pos, a.typ);
        accum
    });

    for i in 0..5 {
        for j in 0..13 {
            if is_occupiable_space((i, j)) {
                if let Some(typ) = by_pos.get(&(i, j)) {
                    print!("{}", typ.as_char());
                } else {
                    print!(".");
                }
            } else {
                print!("#");
            }
        }

        print!("\n");
    }
}

fn main() {
    let src = r"
#############
#...........#
###D#C#D#B###
  #C#A#A#B#
  #########
";
    let state = parse(src);

    let mut pq: BinaryHeap<Node> = BinaryHeap::new();
    pq.push(Node {
        state: state.clone(),
        cost: 0,
    });

    let mut lowest_costs: HashMap<Vec<Amphipod>, i32> = HashMap::new();
    lowest_costs.insert(state.clone(), 0);

    while let Some(node) = pq.pop() {
        if is_finished(&node.state) {
            println!("{}", node.cost);
            return;
        }

        if let Some(lowest_cost) = lowest_costs.get(&node.state) {
            if *lowest_cost < node.cost {
                continue;
            }
        }

        for (id, next_pos) in get_next_moves(&node.state) {
            let prev_apod = node.state[id];
            let mut next_state = node.state.clone();
            next_state[id] = Amphipod {
                typ: prev_apod.typ,
                pos: next_pos,
            };

            let cost = node.cost + man_dist(prev_apod.pos, next_pos) * prev_apod.typ.step_cost();
            if let Some(lowest_cost) = lowest_costs.get(&next_state) {
                if *lowest_cost <= cost {
                    continue;
                }
            }
            lowest_costs.insert(next_state.clone(), cost);

            pq.push(Node {
                state: next_state.clone(),
                cost,
            });
        }
    }

    unreachable!();
}
