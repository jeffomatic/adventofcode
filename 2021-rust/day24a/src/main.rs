use std::{
    collections::HashMap,
    io::{self, Read},
};

#[derive(Debug, Clone, Copy)]
enum Register {
    W,
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy)]
struct ALU {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl ALU {
    fn new(zreg: i64) -> ALU {
        ALU {
            w: 0,
            x: 0,
            y: 0,
            z: zreg,
        }
    }

    fn read(&self, reg: Register) -> i64 {
        match reg {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
        }
    }

    fn write(&mut self, reg: Register, v: i64) {
        match reg {
            Register::W => self.w = v,
            Register::X => self.x = v,
            Register::Y => self.y = v,
            Register::Z => self.z = v,
        }
    }

    fn eval_operand(&self, op: Operand) -> i64 {
        match op {
            Operand::Register(reg) => self.read(reg),
            Operand::Imm(v) => v,
        }
    }

    fn eval_prog(&mut self, prog: &Vec<Ins>, input: &Vec<i64>) -> i64 {
        let mut cursor = 0;
        for &ins in prog.iter() {
            match ins {
                Ins::Inp(reg) => {
                    self.write(reg, input[cursor]);
                    cursor += 1;
                }
                Ins::Add(reg, op) => self.write(reg, self.read(reg) + self.eval_operand(op)),
                Ins::Mul(reg, op) => self.write(reg, self.read(reg) * self.eval_operand(op)),
                Ins::Div(reg, op) => self.write(reg, self.read(reg) / self.eval_operand(op)),
                Ins::Mod(reg, op) => self.write(reg, self.read(reg) % self.eval_operand(op)),
                Ins::Eql(reg, op) => self.write(
                    reg,
                    if self.read(reg) == self.eval_operand(op) {
                        1
                    } else {
                        0
                    },
                ),
            }
        }

        self.z
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Register(Register),
    Imm(i64),
}

#[derive(Debug, Clone, Copy)]
enum Ins {
    Inp(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Mod(Register, Operand),
    Div(Register, Operand),
    Eql(Register, Operand),
}

struct Solver {
    progs_by_digit: Vec<Vec<Ins>>,
    cache: HashMap<(usize, i64), Option<i64>>,
    recursive_calls: i32,
    early_outs: i32,
}

impl Solver {
    fn num_digits(&self) -> usize {
        self.progs_by_digit.len()
    }

    fn best_suffix(&mut self, ndigit: usize, prev_z: i64) -> Option<i64> {
        if ndigit >= self.num_digits() {
            if prev_z == 0 {
                return Some(0);
            }

            return None;
        }

        if let Some(&cached) = self.cache.get(&(ndigit, prev_z)) {
            self.early_outs += 1;
            return cached;
        }

        for input_guess in (1..=9).rev() {
            let next_z =
                ALU::new(prev_z).eval_prog(&self.progs_by_digit[ndigit], &vec![input_guess]);
            self.recursive_calls += 1;
            if let Some(best_suffix) = self.best_suffix(ndigit + 1, next_z) {
                let exp = self.num_digits() - ndigit - 1;
                let new_suffix = 10_i64.pow(exp as u32) * input_guess + best_suffix;

                self.cache.insert((ndigit, prev_z), Some(new_suffix));
                return Some(new_suffix);
            }
        }

        self.cache.insert((ndigit, prev_z), None);
        None
    }
}

fn main() {
    let program = parse(&get_input());
    let mut progs_by_digit = Vec::new();
    let mut all_ins_iter = program.iter();
    let num_digits = 14;
    for _ in 0..num_digits {
        let mut prog = Vec::new();
        for _ in 0..(program.len() / num_digits) {
            prog.push(*all_ins_iter.next().unwrap());
        }
        progs_by_digit.push(prog);
    }

    let mut solver = Solver {
        progs_by_digit,
        cache: HashMap::new(),
        recursive_calls: 0,
        early_outs: 0,
    };
    let res = solver.best_suffix(0, 0).unwrap();

    println!(
        "{} recursive calls: {} early outs {}",
        res, solver.recursive_calls, solver.early_outs
    );
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}

fn parse_register(src: &str) -> Register {
    match src {
        "w" => Register::W,
        "x" => Register::X,
        "y" => Register::Y,
        "z" => Register::Z,
        _ => panic!("invalid register {}", src),
    }
}

fn parse_operand(src: &str) -> Operand {
    match src {
        "w" => Operand::Register(Register::W),
        "x" => Operand::Register(Register::X),
        "y" => Operand::Register(Register::Y),
        "z" => Operand::Register(Register::Z),
        _ => Operand::Imm(src.parse::<i64>().unwrap()),
    }
}

fn parse(src: &str) -> Vec<Ins> {
    src.lines()
        .map(|line| {
            let mut toks = line.split_ascii_whitespace();
            match toks.next().unwrap() {
                "inp" => Ins::Inp(parse_register(toks.next().unwrap())),
                "add" => Ins::Add(
                    parse_register(toks.next().unwrap()),
                    parse_operand(toks.next().unwrap()),
                ),
                "mul" => Ins::Mul(
                    parse_register(toks.next().unwrap()),
                    parse_operand(toks.next().unwrap()),
                ),
                "div" => Ins::Div(
                    parse_register(toks.next().unwrap()),
                    parse_operand(toks.next().unwrap()),
                ),
                "mod" => Ins::Mod(
                    parse_register(toks.next().unwrap()),
                    parse_operand(toks.next().unwrap()),
                ),
                "eql" => Ins::Eql(
                    parse_register(toks.next().unwrap()),
                    parse_operand(toks.next().unwrap()),
                ),
                _ => panic!("could not parse {}", line),
            }
        })
        .collect()
}
