use std::io::{self, Read};

#[derive(Debug, Clone)]
enum Tree {
    Pair(Box<Tree>, Box<Tree>),
    Leaf(u32),
}

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

fn consume_character(s: &str, c: char) -> &str {
    assert!(s.starts_with(c));
    &s[1..]
}

fn consume_number_literal(s: &str) -> (u32, &str) {
    let c = s.chars().next().unwrap();
    let val = c.to_digit(10).unwrap();
    (val, &s[1..])
}

fn parse(s: &str) -> (Tree, &str) {
    if s.starts_with("[") {
        let s = consume_character(s, '[');
        let (left_subtree, s) = parse(s);
        let s = consume_character(s, ',');
        let (right_subtree, s) = parse(s);
        let s = consume_character(s, ']');
        return (
            Tree::Pair(Box::new(left_subtree), Box::new(right_subtree)),
            s,
        );
    } else {
        let (num, s) = consume_number_literal(s);
        return (Tree::Leaf(num), s);
    }
}

fn make_vec_tree(t: &Tree, depth: u32, vt: &mut VecTree) {
    match t {
        Tree::Pair(left, right) => {
            make_vec_tree(left, depth + 1, vt);
            make_vec_tree(right, depth + 1, vt);
        }
        Tree::Leaf(val) => {
            vt.vals.push(*val);
            vt.depths.push(depth - 1);
        }
    }
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
    let trees: Vec<Tree> = get_input().lines().map(|line| parse(line).0).collect();
    let mut best_score = 0;

    for i in 0..trees.len() {
        for j in 0..trees.len() {
            let mut it = VecTree::new();
            make_vec_tree(&trees[i], 0, &mut it);

            let mut jt = VecTree::new();
            make_vec_tree(&trees[j], 0, &mut jt);

            add_array_tree(&mut it, &mut jt);
            reduce(&mut it);

            best_score = best_score.max(score(&mut it));
        }
    }

    println!("{}", best_score);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
