use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn param_value(mem: &Vec<i64>, addr: usize, immediate: bool) -> i64 {
    let v = mem[addr];
    if immediate {
        v
    } else {
        if v < 0 {
            panic!(
                "address {} has contains negative address value: {}",
                addr, v
            )
        }
        mem[v as usize]
    }
}

fn run(mem: &Vec<i64>, input: &Vec<i64>) -> Vec<i64> {
    let mut mem = mem.to_owned();
    let mut input = input.to_owned();
    let mut output = Vec::new();
    let mut ip: usize = 0;

    loop {
        let modes_op = mem[ip];
        let immediate_params = [
            (modes_op % 1000) >= 100,
            (modes_op % 10000) >= 1000,
            modes_op >= 10000,
        ];

        match modes_op % 100 {
            // add
            1 => {
                if immediate_params[2] {
                    panic!("address {}: invalid opcode {}", ip, modes_op)
                }

                let a = param_value(&mem, ip + 1, immediate_params[0]);
                let b = param_value(&mem, ip + 2, immediate_params[1]);
                let dst = mem[ip + 3] as usize;
                mem[dst] = a + b;
                ip += 4;
            }
            // multiply
            2 => {
                if immediate_params[2] {
                    panic!("address {}: invalid opcode {}", ip, modes_op)
                }

                let a = param_value(&mem, ip + 1, immediate_params[0]);
                let b = param_value(&mem, ip + 2, immediate_params[1]);
                let dst = mem[ip + 3] as usize;
                mem[dst] = a * b;
                ip += 4;
            }
            // read input
            3 => {
                let dst = mem[ip + 1] as usize;
                mem[dst] = input.pop().unwrap();
                ip += 2;
            }
            // write output
            4 => {
                let src = mem[ip + 1] as usize;
                output.push(mem[src]);
                ip += 2;
            }
            // jump-if-nonzero
            5 => {
                let a = param_value(&mem, ip + 1, immediate_params[0]);
                let b = param_value(&mem, ip + 2, immediate_params[1]);
                ip = if a != 0 { b as usize } else { ip + 3 }
            }
            // jump-if-zero
            6 => {
                let a = param_value(&mem, ip + 1, immediate_params[0]);
                let b = param_value(&mem, ip + 2, immediate_params[1]);
                ip = if a == 0 { b as usize } else { ip + 3 };
            }
            // less than
            7 => {
                if immediate_params[2] {
                    panic!("address {}: invalid opcode {}", ip, modes_op)
                }

                let a = param_value(&mem, ip + 1, immediate_params[0]);
                let b = param_value(&mem, ip + 2, immediate_params[1]);
                let dst = mem[ip + 3] as usize;
                mem[dst] = if a < b { 1 } else { 0 };
                ip += 4;
            }
            // equal
            8 => {
                if immediate_params[2] {
                    panic!("address {}: invalid opcode {}", ip, modes_op)
                }

                let a = param_value(&mem, ip + 1, immediate_params[0]);
                let b = param_value(&mem, ip + 2, immediate_params[1]);
                let dst = mem[ip + 3] as usize;
                mem[dst] = if a == b { 1 } else { 0 };
                ip += 4;
            }
            // exit
            99 => return output,
            // default
            _ => panic!("address {}: invalid opcode {}", ip, modes_op),
        };
    }
}

fn main() {
    let mem: Vec<i64> = get_input().split(",").map(|s| s.parse().unwrap()).collect();
    let output = run(&mem, &vec![5]);
    println!("{:?}", output)
}
