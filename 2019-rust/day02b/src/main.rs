use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}

fn main() {
    // execute instructions
    let prog: Vec<usize> = get_input().split(",").map(|s| s.parse().unwrap()).collect();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut mem = prog.to_owned();
            mem[1] = noun;
            mem[2] = verb;

            let mut ip = 0;
            'execution: loop {
                match mem[ip] {
                    1 => {
                        let a = mem[mem[ip + 1]];
                        let b = mem[mem[ip + 2]];
                        let dst = mem[ip + 3];
                        mem[dst] = a + b;
                    }
                    2 => {
                        let a = mem[mem[ip + 1]];
                        let b = mem[mem[ip + 2]];
                        let dst = mem[ip + 3];
                        mem[dst] = a * b;
                    }
                    99 => {
                        if mem[0] == 19690720 {
                            println!("{}", (100 * noun) + verb);
                            return;
                        }
                        break 'execution;
                    }
                    other => panic!("invalid opcode {}", other),
                }

                ip += 4;
            }
        }
    }
}
