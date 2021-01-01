use std::collections::VecDeque;

#[derive(Clone)]
pub struct Computer {
    mem: Vec<i64>,
    ip: usize,
    relative_base: usize,
}

#[derive(Copy, Clone, Debug)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Copy, Clone, Debug)]
pub enum State {
    BlockedOnRead,
    Halted,
}

#[derive(Clone, Debug)]
pub struct RunResult {
    pub state: State,
    pub unused_input: Vec<i64>,
    pub output: Vec<i64>,
}

fn param_mode_from_int(v: i64) -> ParamMode {
    match v {
        0 => ParamMode::Position,
        1 => ParamMode::Immediate,
        2 => ParamMode::Relative,
        _ => panic!("invalid param mode {}", v),
    }
}

fn parse_instruction(instruction: i64) -> (i64, [ParamMode; 3]) {
    (
        instruction % 100,
        [
            param_mode_from_int((instruction % 1000) / 100),
            param_mode_from_int((instruction % 10000) / 1000),
            param_mode_from_int((instruction % 100000) / 10000),
        ],
    )
}

pub fn program_from_string(s: &String) -> Vec<i64> {
    s.trim()
        .to_string()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn stream_from_string(s: &str) -> Vec<i64> {
    s.chars().map(|c| c as u8 as i64).collect()
}

pub fn stream_to_string(v: &Vec<i64>) -> String {
    v.iter().map(|n| *n as u8 as char).collect()
}

impl Computer {
    pub fn new(program: &Vec<i64>) -> Computer {
        let mut mem = vec![0; 100000];
        for i in 0..program.len() {
            mem[i] = program[i];
        }

        Computer {
            mem: mem,
            ip: 0,
            relative_base: 0,
        }
    }

    fn param_as_val(&self, addr: usize, mode: ParamMode) -> i64 {
        let v = self.mem[addr];
        match mode {
            ParamMode::Position => self.mem[v as usize],
            ParamMode::Immediate => v,
            ParamMode::Relative => self.mem[(self.relative_base as i64 + v) as usize],
        }
    }

    fn param_as_dst(&self, addr: usize, mode: ParamMode) -> usize {
        match mode {
            ParamMode::Position => self.mem[addr] as usize,
            ParamMode::Immediate => panic!("immediate cannot be destination"),
            ParamMode::Relative => (self.relative_base as i64 + self.mem[addr]) as usize,
        }
    }

    pub fn run(&mut self, input: &Vec<i64>) -> RunResult {
        let mut input_q: VecDeque<i64> = input.iter().cloned().collect();
        let mut output = Vec::new();

        loop {
            let instruction = self.mem[self.ip];
            let (opcode, param_modes) = parse_instruction(instruction);

            match opcode {
                // add
                1 => {
                    let a = self.param_as_val(self.ip + 1, param_modes[0]);
                    let b = self.param_as_val(self.ip + 2, param_modes[1]);
                    let dst = self.param_as_dst(self.ip + 3, param_modes[2]);
                    self.mem[dst] = a + b;
                    self.ip += 4;
                }
                // multiply
                2 => {
                    let a = self.param_as_val(self.ip + 1, param_modes[0]);
                    let b = self.param_as_val(self.ip + 2, param_modes[1]);
                    let dst = self.param_as_dst(self.ip + 3, param_modes[2]);
                    self.mem[dst] = a * b;
                    self.ip += 4;
                }
                // read input
                3 => {
                    // block waiting for input
                    if input_q.is_empty() {
                        return RunResult {
                            state: State::BlockedOnRead,
                            unused_input: Vec::from(input_q),
                            output: output,
                        };
                    }

                    let dst = self.param_as_dst(self.ip + 1, param_modes[0]);
                    self.mem[dst] = input_q.pop_front().unwrap();
                    self.ip += 2;
                }
                // write output
                4 => {
                    let a = self.param_as_val(self.ip + 1, param_modes[0]);
                    output.push(a);
                    self.ip += 2;
                }
                // jump-if-nonzero
                5 => {
                    let a = self.param_as_val(self.ip + 1, param_modes[0]);
                    let b = self.param_as_val(self.ip + 2, param_modes[1]);
                    self.ip = if a != 0 { b as usize } else { self.ip + 3 }
                }
                // jump-if-zero
                6 => {
                    let a = self.param_as_val(self.ip + 1, param_modes[0]);
                    let b = self.param_as_val(self.ip + 2, param_modes[1]);
                    self.ip = if a == 0 { b as usize } else { self.ip + 3 };
                }
                // less than
                7 => {
                    let a = self.param_as_val(self.ip + 1, param_modes[0]);
                    let b = self.param_as_val(self.ip + 2, param_modes[1]);
                    let dst = self.param_as_dst(self.ip + 3, param_modes[2]);
                    self.mem[dst] = if a < b { 1 } else { 0 };
                    self.ip += 4;
                }
                // equal
                8 => {
                    let a = self.param_as_val(self.ip + 1, param_modes[0]);
                    let b = self.param_as_val(self.ip + 2, param_modes[1]);
                    let dst = self.param_as_dst(self.ip + 3, param_modes[2]);
                    self.mem[dst] = if a == b { 1 } else { 0 };
                    self.ip += 4;
                }
                // set relative base
                9 => {
                    self.relative_base = ((self.relative_base as i64)
                        + self.param_as_val(self.ip + 1, param_modes[0]))
                        as usize;
                    self.ip += 2;
                }
                // exit
                99 => {
                    return RunResult {
                        state: State::Halted,
                        unused_input: Vec::from(input_q),
                        output: output,
                    }
                }
                // default
                _ => panic!("address {}: invalid opcode {}", self.ip, instruction),
            };
        }
    }
}
