use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() {
    let instructions: Vec<Instruction> = get_input()
        .lines()
        .map(parse_instruction)
        .filter(|ins| {
            range_overlap(ins.x, (-50, 50))
                && range_overlap(ins.y, (-50, 50))
                && range_overlap(ins.z, (-50, 50))
        })
        .map(|ins| {
            let mut clamped = ins;
            clamped.x.0 = clamped.x.0.max(-50);
            clamped.x.1 = clamped.x.1.min(50);
            clamped.y.0 = clamped.y.0.max(-50);
            clamped.y.1 = clamped.y.1.min(50);
            clamped.z.0 = clamped.z.0.max(-50);
            clamped.z.1 = clamped.z.1.min(50);
            clamped
        })
        .collect();

    let mut active = HashSet::new();
    for ins in instructions {
        for x in ins.x.0..=ins.x.1 {
            for y in ins.y.0..=ins.y.1 {
                for z in ins.z.0..=ins.z.1 {
                    if ins.on {
                        active.insert((x, y, z));
                    } else {
                        active.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    println!("{}", active.len());
}

#[derive(Debug, Clone)]
struct Instruction {
    on: bool,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

fn range_overlap(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

// parses expressions of the following form: x=-24..25
fn parse_range(mut src: &str) -> (i32, i32) {
    // ignore `x=`
    src = &src[2..];
    let mut toks = src.split("..");
    (
        toks.next().unwrap().parse().unwrap(),
        toks.next().unwrap().parse().unwrap(),
    )
}

// parses expression of the following form:
//  on x=-24..25,y=-36..8,z=-15..31
//  off x=-39..-20,y=-32..-18,z=36..47
fn parse_instruction(mut line: &str) -> Instruction {
    let on = if line.starts_with("on ") {
        line = &line[3..];
        true
    } else {
        line = &line[4..];
        false
    };

    let mut toks = line.split(",");

    Instruction {
        on,
        x: parse_range(toks.next().unwrap()),
        y: parse_range(toks.next().unwrap()),
        z: parse_range(toks.next().unwrap()),
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
