use std::io::{self, Read};

#[derive(Debug)]
enum Opcode {
    Add,
    Mul,
    Read,
    Print,
    Exit,
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}

fn param_value(mem: &Vec<i64>, addr: usize, immediate: bool) -> i64 {
    let v = mem[addr];
    if immediate {
        return v;
    }
    if v < 0 {
        panic!(
            "address {} has contains negative address value: {}",
            addr, v
        )
    }
    mem[v as usize]
}

fn run(mem: &Vec<i64>, input: &Vec<i64>) -> Vec<i64> {
    let mut mem = mem.to_owned();
    let mut input = input.to_owned();
    let mut output = Vec::new();
    let mut ip: usize = 0;

    loop {
        let modes_op = mem[ip];
        let opcode = match modes_op % 100 {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            3 => Opcode::Read,
            4 => Opcode::Print,
            99 => Opcode::Exit,
            _ => panic!("address {}: invalid opcode {}", ip, modes_op),
        };
        let immediate_params = [
            (modes_op % 1000) >= 100,
            (modes_op % 10000) >= 1000,
            modes_op >= 10000,
        ];

        match opcode {
            Opcode::Add => {
                if immediate_params[2] {
                    panic!("address {}: invalid opcode {}", ip, modes_op)
                }

                let a = param_value(&mem, ip + 1, immediate_params[0]);
                let b = param_value(&mem, ip + 2, immediate_params[1]);
                let dst = mem[ip + 3] as usize;
                mem[dst] = a + b;
                ip += 4;
            }
            Opcode::Mul => {
                if immediate_params[2] {
                    panic!("address {}: invalid opcode {}", ip, modes_op)
                }

                let a = param_value(&mem, ip + 1, immediate_params[0]);
                let b = param_value(&mem, ip + 2, immediate_params[1]);
                let dst = mem[ip + 3] as usize;
                mem[dst] = a * b;
                ip += 4;
            }
            Opcode::Read => {
                let dst = mem[ip + 1] as usize;
                mem[dst] = input.pop().unwrap();
                ip += 2;
            }
            Opcode::Print => {
                let src = mem[ip + 1] as usize;
                output.push(mem[src]);
                ip += 2;
            }
            Opcode::Exit => return output,
        };
    }
}

fn main() {
    let mem: Vec<i64> = get_input().split(",").map(|s| s.parse().unwrap()).collect();
    let output = run(&mem, &vec![1]);
    println!("{:?}", output)
}
