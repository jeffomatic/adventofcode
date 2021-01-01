use permutohedron::Heap;
use std::cmp;
use std::collections::VecDeque;
use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

struct Simulation {
    mem: Vec<i64>,
    ip: usize,
    halted: bool,
}

impl Simulation {
    fn new(prog: &Vec<i64>) -> Simulation {
        Simulation {
            mem: prog.to_owned(),
            ip: 0,
            halted: false,
        }
    }

    fn param_val(&self, addr: usize, immediate: bool) -> i64 {
        let v = self.mem[addr];
        if immediate {
            return v;
        }
        self.mem[v as usize]
    }

    fn run(&mut self, input: &mut VecDeque<i64>, output: &mut VecDeque<i64>) {
        if self.halted {
            return;
        }

        loop {
            let modes_op = self.mem[self.ip];
            let immediate_params = [
                (modes_op % 1000) >= 100,
                (modes_op % 10000) >= 1000,
                modes_op >= 10000,
            ];

            match modes_op % 100 {
                // add
                1 => {
                    if immediate_params[2] {
                        panic!("address {}: invalid opcode {}", self.ip, modes_op)
                    }

                    let a = self.param_val(self.ip + 1, immediate_params[0]);
                    let b = self.param_val(self.ip + 2, immediate_params[1]);
                    let dst = self.mem[self.ip + 3] as usize;
                    self.mem[dst] = a + b;
                    self.ip += 4;
                }
                // multiply
                2 => {
                    if immediate_params[2] {
                        panic!("address {}: invalid opcode {}", self.ip, modes_op)
                    }

                    let a = self.param_val(self.ip + 1, immediate_params[0]);
                    let b = self.param_val(self.ip + 2, immediate_params[1]);
                    let dst = self.mem[self.ip + 3] as usize;
                    self.mem[dst] = a * b;
                    self.ip += 4;
                }
                // read input
                3 => {
                    if input.is_empty() {
                        return;
                    }

                    let dst = self.mem[self.ip + 1] as usize;
                    self.mem[dst] = input.pop_front().unwrap();
                    self.ip += 2;
                }
                // write output
                4 => {
                    let src = self.mem[self.ip + 1] as usize;
                    output.push_back(self.mem[src]);
                    self.ip += 2;
                }
                // jump-if-nonzero
                5 => {
                    let a = self.param_val(self.ip + 1, immediate_params[0]);
                    let b = self.param_val(self.ip + 2, immediate_params[1]);
                    self.ip = if a != 0 { b as usize } else { self.ip + 3 }
                }
                // jump-if-zero
                6 => {
                    let a = self.param_val(self.ip + 1, immediate_params[0]);
                    let b = self.param_val(self.ip + 2, immediate_params[1]);
                    self.ip = if a == 0 { b as usize } else { self.ip + 3 };
                }
                // less than
                7 => {
                    if immediate_params[2] {
                        panic!("address {}: invalid opcode {}", self.ip, modes_op)
                    }

                    let a = self.param_val(self.ip + 1, immediate_params[0]);
                    let b = self.param_val(self.ip + 2, immediate_params[1]);
                    let dst = self.mem[self.ip + 3] as usize;
                    self.mem[dst] = if a < b { 1 } else { 0 };
                    self.ip += 4;
                }
                // equal
                8 => {
                    if immediate_params[2] {
                        panic!("address {}: invalid opcode {}", self.ip, modes_op)
                    }

                    let a = self.param_val(self.ip + 1, immediate_params[0]);
                    let b = self.param_val(self.ip + 2, immediate_params[1]);
                    let dst = self.mem[self.ip + 3] as usize;
                    self.mem[dst] = if a == b { 1 } else { 0 };
                    self.ip += 4;
                }
                // exit
                99 => {
                    self.halted = true;
                    return;
                }
                // default
                _ => panic!("address {}: invalid opcode {}", self.ip, modes_op),
            };
        }
    }
}

// To my current knowledge, this is the only way to get mutable references of
// more than one element in a mutable vec. It has the whiff of a hack and makes
// me feel a little uncomfortable about the language.
//
// See: https://www.reddit.com/r/rust/comments/3jxe4z/mutable_references_to_separate_indices_of_a_vector/
fn vec_get_mut_multi<T>(v: &mut Vec<T>, mut i: usize, mut j: usize) -> (&mut T, &mut T) {
    if i == j {
        panic!("i and j are the same");
    }

    let swap = i > j;
    if swap {
        let tmp = i;
        i = j;
        j = tmp;
    }

    let mid = ((i + j) / 2) + 1;
    let (head, tail) = v.split_at_mut(mid);
    let a = &mut head[i];
    let b = &mut tail[j - mid];

    if swap {
        (b, a)
    } else {
        (a, b)
    }
}

fn simulate(prog: &Vec<i64>, phases: &Vec<i64>) -> i64 {
    let mut sims: Vec<Simulation> = phases.iter().map(|_| Simulation::new(prog)).collect();
    let mut signals: Vec<VecDeque<i64>> = phases
        .iter()
        .map(|p| {
            let mut q = VecDeque::new();
            q.push_back(*p);
            q
        })
        .collect();

    // set initial signal
    signals[0].push_back(0);

    let mut last = -1;
    loop {
        if sims.iter().all(|s| s.halted) {
            return last;
        }

        for i in 0..sims.len() {
            let j = (i + 1) % sims.len();
            let (inq, outq) = vec_get_mut_multi(&mut signals, i, j);
            sims[i].run(inq, outq);
        }

        // track values emitted by the last simulation
        if let Some(v) = signals[0].back() {
            last = *v;
        }
    }
}

fn main() {
    let prog: Vec<i64> = get_input().split(",").map(|s| s.parse().unwrap()).collect();
    let mut phases: Vec<i64> = (5..=9).collect();
    let best = Heap::new(&mut phases).fold(-1, |best, p| cmp::max(best, simulate(&prog, &p)));
    println!("{}", best)
}
