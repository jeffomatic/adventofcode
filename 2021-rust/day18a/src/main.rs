use std::io::{self, Read};

#[derive(Debug, Clone)]
struct VecTree {
    vals: Vec<u32>,
    depths: Vec<u32>,
}

impl VecTree {
    fn parse(s: &str) -> VecTree {
        let mut t = VecTree {
            vals: Vec::new(),
            depths: Vec::new(),
        };

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

    fn explode(&mut self) -> bool {
        for i in 0..self.depths.len() {
            let depth = self.depths[i];
            if depth != 4 {
                continue;
            }

            // add left value to left neighbor
            if i != 0 {
                self.vals[i - 1] += self.vals[i];
            }

            // add right value to right neighbor
            if i + 2 < self.vals.len() {
                self.vals[i + 2] += self.vals[i + 1];
            }

            self.vals[i] = 0;
            self.depths[i] = 3;
            self.vals.remove(i + 1);
            self.depths.remove(i + 1);

            return true;
        }

        false
    }

    fn split(&mut self) -> bool {
        for i in 0..self.vals.len() {
            let v = self.vals[i];
            if v < 10 {
                continue;
            }

            let (a, b) = if v % 2 == 0 {
                (v / 2, v / 2)
            } else {
                (v / 2, v / 2 + 1)
            };

            self.vals[i] = a;
            self.depths[i] += 1;
            self.vals.insert(i + 1, b);
            self.depths.insert(i + 1, self.depths[i]);

            return true;
        }

        false
    }

    fn reduce(&mut self) {
        loop {
            if !self.explode() && !self.split() {
                break;
            }
        }
    }

    fn add(&mut self, other: &VecTree) {
        self.vals.extend(other.vals.iter());
        self.depths.extend(other.depths.iter());
        for i in 0..self.depths.len() {
            self.depths[i] += 1;
        }
    }

    fn score(&self) -> u32 {
        let mut vals = self.vals.clone();
        let mut depths = self.depths.clone();

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
}

fn main() {
    let input = get_input();
    let mut lines = input.lines();

    let mut tree = VecTree::parse(lines.next().unwrap());
    for line in lines {
        tree.add(&VecTree::parse(line));
        tree.reduce();
    }

    println!("{}", tree.score());
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
