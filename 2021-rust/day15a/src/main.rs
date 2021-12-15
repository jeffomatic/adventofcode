use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    io::{self, Read},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    pos: (usize, usize),
    cost: u32,
}

fn neighbors_of(pos: (usize, usize), num_rows: usize, num_cols: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    // North neighbor
    if pos.0 > 0 {
        res.push((pos.0 - 1, pos.1));
    }

    // South neighbor
    if pos.0 < num_rows - 1 {
        res.push((pos.0 + 1, pos.1));
    }

    // West neighbor
    if pos.1 > 0 {
        res.push((pos.0, pos.1 - 1));
    }

    // East neighbor
    if pos.1 < num_cols - 1 {
        res.push((pos.0, pos.1 + 1));
    }

    return res;
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let grid: Vec<Vec<u32>> = get_input()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut pq: BinaryHeap<Node> = BinaryHeap::new();
    pq.push(Node {
        pos: (0, 1),
        cost: grid[0][1],
    });
    pq.push(Node {
        pos: (1, 0),
        cost: grid[1][0],
    });

    let mut lowest_costs: HashMap<(usize, usize), u32> = HashMap::new();
    lowest_costs.insert((0, 0), 0);

    while let Some(node) = pq.pop() {
        if node.pos == (num_rows - 1, num_cols - 1) {
            println!("{}", node.cost);
            return;
        }

        if let Some(lowest_cost) = lowest_costs.get(&node.pos) {
            if *lowest_cost <= node.cost {
                continue;
            }
        }

        for neighbor in neighbors_of(node.pos, num_rows, num_cols) {
            let cost = node.cost + grid[neighbor.0][neighbor.1];
            if let Some(lowest_cost) = lowest_costs.get(&neighbor) {
                if *lowest_cost <= cost {
                    continue;
                }
            }

            pq.push(Node {
                pos: neighbor,
                cost,
            });

            lowest_costs.insert(node.pos, node.cost);
        }
    }

    unreachable!();
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
