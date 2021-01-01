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
        &mut vec!['R', 'U', 'N', '\n']
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
        println!(
            "{}",
            result
                .output
                .iter()
                .map(|n| *n as u8 as char)
                .collect::<String>()
        );

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

    // Jump if:
    // - if it's safe to make a preemptive jump, i.e. either B or C is false,
    //   and you can make the next two jumps (D and H are both true)
    // - you have to, i.e. !A
    // ((!B || !C) && (D && H)) || !A
    let script = parse_string_script(
        "NOT B J
NOT C T
OR J T
NOT D J
NOT J J
AND J T
NOT H J
NOT J J
AND J T
NOT A J
OR T J",
    );
    println!("{}", try_script(&program, &script).unwrap());
}
