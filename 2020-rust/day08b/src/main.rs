use std::{
    collections::HashSet,
    io::{self, Read},
    str::FromStr,
    unreachable,
};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Opcode {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    opcode: Opcode,
    operand: i64,
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r#"^(?P<opcode>acc|jmp|nop) (?P<opsign>\+|-)(?P<opval>\d+)$"#).unwrap();
        }

        let caps = RE.captures(s).unwrap();

        let opcode = match &caps["opcode"] {
            "acc" => Opcode::Acc,
            "jmp" => Opcode::Jmp,
            "nop" => Opcode::Nop,
            _ => unreachable!(),
        };

        let sign: i64 = match &caps["opsign"] {
            "+" => 1,
            "-" => -1,
            _ => unreachable!(),
        };

        let opval: i64 = caps["opval"].parse().unwrap();

        Ok(Instruction {
            opcode: opcode,
            operand: sign * opval,
        })
    }
}

fn execute(instructions: Vec<Instruction>) -> Option<i64> {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut pc: usize = 0;
    let mut acc: i64 = 0;

    loop {
        visited.insert(pc);
        let ins = &instructions[pc];
        let mut jmpsize: i64 = 1;

        match ins.opcode {
            Opcode::Acc => acc += ins.operand,
            Opcode::Nop => (),
            Opcode::Jmp => jmpsize = ins.operand,
        }

        pc = (pc as i64 + jmpsize) as usize;

        if visited.contains(&pc) {
            return None;
        }

        if pc == instructions.len() {
            return Some(acc);
        }
    }
}

fn main() {
    let instructions: Vec<Instruction> = get_input()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    for i in 0..instructions.len() {
        let mut modified = instructions.clone();
        match modified[i].opcode {
            Opcode::Acc => continue,
            Opcode::Jmp => modified[i].opcode = Opcode::Nop,
            Opcode::Nop => modified[i].opcode = Opcode::Jmp,
        }

        if let Some(res) = execute(modified) {
            println!("{}", res);
            break;
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
