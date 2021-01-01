use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

struct Computer {
    mem: Vec<i64>,
    ip: usize,
    relative_base: usize,
    state: State,
}

#[derive(Copy, Clone, Debug)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Copy, Clone, Debug)]
enum State {
    Ready,
    BlockedOnRead,
    Halted,
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

impl Computer {
    fn new(program: &Vec<i64>) -> Computer {
        let mut mem = vec![0; 100000];
        for i in 0..program.len() {
            mem[i] = program[i];
        }

        Computer {
            mem: mem,
            ip: 0,
            relative_base: 0,
            state: State::Ready,
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

    fn run(&mut self, input: &mut VecDeque<i64>, output: &mut VecDeque<i64>) {
        match self.state {
            State::Ready => (),
            State::BlockedOnRead => {
                if input.is_empty() {
                    return;
                }
                self.state = State::Ready;
            }
            State::Halted => return,
        }

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
                    if input.is_empty() {
                        self.state = State::BlockedOnRead;
                        return;
                    }

                    let dst = self.param_as_dst(self.ip + 1, param_modes[0]);
                    self.mem[dst] = input.pop_front().unwrap();
                    self.ip += 2;
                }
                // write output
                4 => {
                    let a = self.param_as_val(self.ip + 1, param_modes[0]);
                    output.push_back(a);
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
                    self.state = State::Halted;
                    return;
                }
                // default
                _ => panic!("address {}: invalid opcode {}", self.ip, instruction),
            };
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone, Debug)]
enum Turn {
    Left,
    Right,
}

fn change_dir(from: Dir, turn: Turn) -> Dir {
    match from {
        Dir::North => match turn {
            Turn::Left => Dir::West,
            Turn::Right => Dir::East,
        },
        Dir::East => match turn {
            Turn::Left => Dir::North,
            Turn::Right => Dir::South,
        },
        Dir::South => match turn {
            Turn::Left => Dir::East,
            Turn::Right => Dir::West,
        },
        Dir::West => match turn {
            Turn::Left => Dir::South,
            Turn::Right => Dir::North,
        },
    }
}

fn move_pos(from: (i64, i64), dir: Dir) -> (i64, i64) {
    match dir {
        Dir::North => (from.0, from.1 + 1),
        Dir::East => (from.0 + 1, from.1),
        Dir::South => (from.0, from.1 - 1),
        Dir::West => (from.0 - 1, from.1),
    }
}

fn main() {
    let program: Vec<i64> = get_input().split(",").map(|s| s.parse().unwrap()).collect();

    let mut cpu = Computer::new(&program);
    let mut input: VecDeque<i64> = VecDeque::new();
    let mut output: VecDeque<i64> = VecDeque::new();

    let mut dir = Dir::North;
    let mut pos = (0, 0);
    let mut white_nodes = HashSet::new();
    let mut painted = HashSet::new();

    loop {
        match cpu.state {
            State::Ready => cpu.run(&mut input, &mut output),
            State::BlockedOnRead => {
                let color = if white_nodes.contains(&pos) { 1 } else { 0 };
                input.push_back(color);
                cpu.run(&mut input, &mut output);
            }
            State::Halted => break,
        }

        while output.len() >= 2 {
            match output.pop_front().unwrap() {
                0 => {
                    white_nodes.remove(&pos);
                }
                1 => {
                    white_nodes.insert(pos);
                }
                v => {
                    panic!("invalid color {:?}", v);
                }
            }

            painted.insert(pos);

            let turn = match output.pop_front().unwrap() {
                0 => Turn::Left,
                1 => Turn::Right,
                v => panic!("invalid turn {:?}", v),
            };

            dir = change_dir(dir, turn);
            pos = move_pos(pos, dir);
        }
    }

    println!("{}", painted.len())
}
