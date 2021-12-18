use std::io::{self, Read};

#[derive(Debug, Clone)]
struct VecTree {
    vals: Vec<u32>,
    depths: Vec<u32>,
}

impl VecTree {
    fn new() -> VecTree {
        VecTree {
            vals: Vec::new(),
            depths: Vec::new(),
        }
    }
}

fn parse(s: &str) -> VecTree {
    let mut t = VecTree::new();

    let mut depth = 0;
    for c in s.chars() {
        match c {
            '[' => {
                depth += 1;
            }
            ',' => (),
            ']' => {
                depth -= 1;
            }
            d => {
                t.vals.push(d.to_digit(10).unwrap());
                t.depths.push(depth - 1);
            }
        }
    }

    t
}

fn explode(t: &mut VecTree) -> bool {
    for i in 0..t.depths.len() {
        let depth = t.depths[i];
        if depth != 4 {
            continue;
        }

        // add left value to left neighbor
        if i != 0 {
            t.vals[i - 1] += t.vals[i];
        }

        // add right value to right neighbor
        if i + 2 < t.vals.len() {
            t.vals[i + 2] += t.vals[i + 1];
        }

        t.vals[i] = 0;
        t.depths[i] = 3;
        t.vals.remove(i + 1);
        t.depths.remove(i + 1);

        return true;
    }

    false
}

fn split(t: &mut VecTree) -> bool {
    for i in 0..t.vals.len() {
        let v = t.vals[i];
        if v < 10 {
            continue;
        }

        let (a, b) = if v % 2 == 0 {
            (v / 2, v / 2)
        } else {
            (v / 2, v / 2 + 1)
        };

        t.vals[i] = a;
        t.depths[i] += 1;
        t.vals.insert(i + 1, b);
        t.depths.insert(i + 1, t.depths[i]);

        return true;
    }

    false
}

fn reduce(t: &mut VecTree) {
    loop {
        let did_explode = explode(t);
        if did_explode {
            continue;
        }

        let did_split = split(t);
        if !did_explode && !did_split {
            break;
        }
    }
}

fn add_array_tree(a: &mut VecTree, b: &mut VecTree) {
    a.vals.append(&mut b.vals);
    a.depths.append(&mut b.depths);
    for i in 0..a.depths.len() {
        a.depths[i] += 1;
    }
}

fn score(t: &mut VecTree) -> u32 {
    while t.vals.len() > 1 {
        for i in 0..t.depths.len() - 1 {
            if t.depths[i] == t.depths[i + 1] {
                t.vals[i] = 3 * t.vals[i] + 2 * t.vals[i + 1];
                t.vals.remove(i + 1);
                t.depths.remove(i + 1);

                if t.depths[i] > 0 {
                    t.depths[i] -= 1;
                }

                break;
            }
        }
    }

    t.vals[0]
}

fn main() {
    let trees: Vec<VecTree> = get_input().lines().map(parse).collect();

    let mut best_score = 0;
    for i in trees.iter() {
        for j in trees.iter() {
            let mut a = i.clone();
            let mut b = j.clone();
            add_array_tree(&mut a, &mut b);
            reduce(&mut a);
            best_score = best_score.max(score(&mut a));
        }
    }

    println!("{}", best_score);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
