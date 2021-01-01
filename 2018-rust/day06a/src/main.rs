use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read};

fn get_input() -> String {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();
    handle.read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}

fn neighbors(p: (i32, i32), min: (i32, i32), max: (i32, i32)) -> HashSet<(i32, i32)> {
    vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .map(|s| (p.0 + s.0, p.1 + s.1))
        .filter(|vp| min.0 <= vp.0 && vp.0 <= max.0)
        .filter(|vp| min.1 <= vp.1 && vp.1 <= max.1)
        .collect()
}

fn main() {
    let points: HashSet<_> = get_input()
        .lines()
        .map(|s| s.split(", "))
        .map(|mut toks| {
            (
                toks.next().unwrap().parse::<i32>().unwrap(),
                toks.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    let mut min = (i32::max_value(), i32::max_value());
    let mut max = (0, 0);
    for p in points.iter() {
        if p.0 < min.0 {
            min = (p.0, min.1);
        }
        if p.0 > max.0 {
            max = (p.0, max.1);
        }
        if p.1 < min.1 {
            min = (min.0, p.1);
        }
        if p.1 > max.1 {
            max = (max.0, p.1);
        }
    }

    let mut ownership = HashMap::new();
    let mut to_visit = VecDeque::new();
    for p in points.iter() {
        ownership.insert(*p, (*p, 0));

        for n in neighbors(*p, min, max) {
            to_visit.push_back((n, *p, 1));
        }
    }

    let mut shared_points = HashSet::new();
    while !to_visit.is_empty() {
        let (p, owner, dist) = to_visit.pop_front().unwrap();
        match ownership.get(&p) {
            Some((other_owner, other_dist)) => {
                if *other_dist == dist && *other_owner != owner {
                    shared_points.insert(p);
                }
            }
            None => {
                ownership.insert(p, (owner, dist));

                // Continue for sole owner of point
                for n in neighbors(p, min, max) {
                    to_visit.push_back((n, owner, dist + 1));
                }
            }
        };
    }

    let mut score_by_owner = HashMap::new();
    let mut invalid_winners = HashSet::new();
    for (p, (owner, _)) in ownership {
        if p.0 == min.0 || p.0 == max.0 || p.1 == min.1 || p.1 == max.1 {
            invalid_winners.insert(owner);
            continue;
        }

        if shared_points.contains(&p) {
            continue;
        }

        let score = match score_by_owner.get(&owner) {
            Some(s) => *s,
            None => 0,
        };
        score_by_owner.insert(owner, score + 1);
    }

    let mut max_score = 0;
    for (owner, score) in score_by_owner {
        if invalid_winners.contains(&owner) {
            continue;
        }

        if score > max_score {
            max_score = score;
        }
    }

    println!("{}", max_score);
}
