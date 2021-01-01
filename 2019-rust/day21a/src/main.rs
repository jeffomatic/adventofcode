use intcode;
use std::io::{self, Read};

fn parse_string_instruction(s: &str) -> Vec<i64> {
    let mut v = s.chars().map(|c| c as u8 as i64).collect::<Vec<i64>>();
    v.push('\n' as u8 as i64);
    v
}

fn try_script(program: &Vec<i64>, script: &Vec<i64>) -> Option<i64> {
    // add WALK instruction
    let mut script = script.to_vec();
    script.append(
        &mut vec!['W', 'A', 'L', 'K', '\n']
            .iter()
            .map(|c| *c as u8 as i64)
            .collect(),
    );

    let mut cpu = intcode::Computer::new(&program);
    cpu.run(&Vec::new()); // clear prompt
    let result = cpu.run(&script);

    let last = *result.output.last().unwrap();
    if last < 0 || 255 < last {
        Some(last)
    } else {
        None
    }
}

fn parse_string_script(s: &str) -> Vec<i64> {
    let i64_lines: Vec<Vec<i64>> = s
        .split("\n")
        .map(|line| parse_string_instruction(line))
        .collect();
    i64_lines.iter().fold(Vec::new(), |mut acc, line| {
        acc.append(&mut line.to_vec());
        acc
    })
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let program: Vec<i64> = input
        .trim()
        .to_string()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let script = parse_string_script(
        "NOT A T
NOT B J
OR J T
NOT C J
OR J T
NOT D J
NOT J J
AND T J",
    );
    println!("{}", try_script(&program, &script).unwrap());
}
