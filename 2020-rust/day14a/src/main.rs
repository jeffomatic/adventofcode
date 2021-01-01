use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

use regex::Regex;

#[derive(Debug)]
enum Op {
    SetMask(HashSet<usize>, HashSet<usize>),
    AssignMem(usize, u64),
}

fn munge_value(mut val: u64, ons: &HashSet<usize>, offs: &HashSet<usize>) -> u64 {
    for i in 0..36 {
        if ons.contains(&i) {
            val |= 1 << i;
        } else if offs.contains(&i) {
            val &= !(1 << i);
        }
    }
    return val;
}

fn main() {
    let mask_re = Regex::new(r#"mask = ([X01]{36})"#).unwrap();
    let assign_re = Regex::new(r#"mem\[(\d+)\] = (\d+)"#).unwrap();

    let program: Vec<Op> = get_input()
        .lines()
        .map(|line| {
            if let Some(caps) = mask_re.captures(line) {
                let mut ons = HashSet::new();
                let mut offs = HashSet::new();
                for (i, c) in caps[1].chars().rev().enumerate() {
                    match c {
                        '0' => {
                            offs.insert(i);
                            ()
                        }
                        '1' => {
                            ons.insert(i);
                            ()
                        }
                        _ => (),
                    }
                }

                return Op::SetMask(ons, offs);
            }

            if let Some(caps) = assign_re.captures(line) {
                let addr: usize = caps[1].parse().unwrap();
                let val: u64 = caps[2].parse().unwrap();
                return Op::AssignMem(addr, val);
            }

            panic!("could not parse {}", line)
        })
        .collect();

    let mut mem: HashMap<usize, u64> = HashMap::new();
    let mut ons: HashSet<usize> = HashSet::new();
    let mut offs: HashSet<usize> = HashSet::new();
    for op in program {
        match op {
            Op::SetMask(mask_ons, mask_offs) => {
                ons = mask_ons;
                offs = mask_offs;
            }
            Op::AssignMem(addr, val) => {
                mem.insert(addr, munge_value(val, &ons, &offs));
            }
        }
    }

    let res = mem.iter().fold(0, |accum, (_, v)| accum + v);
    println!("{}", res)
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
