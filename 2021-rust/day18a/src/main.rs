use std::io::{self, Read};

#[derive(Debug, Clone)]
enum Tree {
    Pair(Box<Tree>, Box<Tree>),
    Leaf(u32),
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

fn make_array_tree(t: &Tree, depth: u32, vals: &mut Vec<u32>, depths: &mut Vec<u32>) {
    match t {
        Tree::Pair(left, right) => {
            make_array_tree(left, depth + 1, vals, depths);
            make_array_tree(right, depth + 1, vals, depths);
        }
        Tree::Leaf(val) => {
            vals.push(*val);
            depths.push(depth - 1);
        }
    }
}

fn explode(vals: &mut Vec<u32>, depths: &mut Vec<u32>) -> bool {
    for i in 0..depths.len() {
        let depth = depths[i];
        if depth != 4 {
            continue;
        }

        // add left value to left neighbor
        if i != 0 {
            vals[i - 1] += vals[i];
        }

        // add right value to right neighbor
        if i + 2 < vals.len() {
            vals[i + 2] += vals[i + 1];
        }

        vals[i] = 0;
        depths[i] = 3;
        vals.remove(i + 1);
        depths.remove(i + 1);

        return true;
    }

    false
}

fn split(vals: &mut Vec<u32>, depths: &mut Vec<u32>) -> bool {
    for i in 0..vals.len() {
        let v = vals[i];
        if v < 10 {
            continue;
        }

        let (a, b) = if v % 2 == 0 {
            (v / 2, v / 2)
        } else {
            (v / 2, v / 2 + 1)
        };

        vals[i] = a;
        depths[i] += 1;
        vals.insert(i + 1, b);
        depths.insert(i + 1, depths[i]);

        return true;
    }

    false
}

fn reduce(vals: &mut Vec<u32>, depths: &mut Vec<u32>) {
    loop {
        let did_explode = explode(vals, depths);
        if did_explode {
            continue;
        }

        let did_split = split(vals, depths);
        if !did_explode && !did_split {
            break;
        }
    }
}

fn add_array_tree(
    a_vals: &mut Vec<u32>,
    a_depths: &mut Vec<u32>,
    b_vals: &mut Vec<u32>,
    b_depths: &mut Vec<u32>,
) {
    a_vals.append(b_vals);
    a_depths.append(b_depths);
    for i in 0..a_depths.len() {
        a_depths[i] += 1;
    }
}

fn score(vals: &mut Vec<u32>, depths: &mut Vec<u32>) -> u32 {
    while vals.len() > 1 {
        for i in 0..depths.len() - 1 {
            if depths[i] == depths[i + 1] {
                vals[i] = 3 * vals[i] + 2 * vals[i + 1];
                vals.remove(i + 1);
                depths.remove(i + 1);

                if depths[i] > 0 {
                    depths[i] -= 1;
                }

                break;
            }
        }
    }

    vals[0]
}

fn main() {
    let input = get_input();
    let mut lines = input.lines();

    let tree = parse(lines.next().unwrap()).0;
    let mut vals = Vec::new();
    let mut depths = Vec::new();
    make_array_tree(&tree, 0, &mut vals, &mut depths);

    for line in lines {
        let other_tree = parse(line).0;
        let mut other_vals = Vec::new();
        let mut other_depths = Vec::new();
        make_array_tree(&other_tree, 0, &mut other_vals, &mut other_depths);
        add_array_tree(&mut vals, &mut depths, &mut other_vals, &mut other_depths);
        reduce(&mut vals, &mut depths);
    }

    println!("{}", score(&mut vals, &mut depths));
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
