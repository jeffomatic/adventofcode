use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}

fn main() {
    // execute instructions
    let mut toks: Vec<usize> = get_input().split(",").map(|s| s.parse().unwrap()).collect();
    toks[1] = 12;
    toks[2] = 2;

    let mut pos = 0;
    loop {
        match toks[pos] {
            1 => {
                let a = toks[toks[pos + 1]];
                let b = toks[toks[pos + 2]];
                let dst = toks[pos + 3];
                toks[dst] = a + b;
            }
            2 => {
                let a = toks[toks[pos + 1]];
                let b = toks[toks[pos + 2]];
                let dst = toks[pos + 3];
                toks[dst] = a * b;
            }
            99 => {
                println!("{}", toks[0]);
                return;
            }
            other => panic!("invalid opcode {}", other),
        }

        pos += 4;
    }
}
