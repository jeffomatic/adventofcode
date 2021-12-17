use std::io::{self, Read};

#[derive(Debug, Clone)]
struct LiteralPacket {
    version: u8,
    val: u64,
}

#[derive(Debug, Clone, Copy)]
enum OpType {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug, Clone)]
struct OperatorPacket {
    version: u8,
    optype: OpType,
    subpackets: Vec<Packet>,
}

#[derive(Debug, Clone)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

fn parse_version(bits: &mut dyn Iterator<Item = char>) -> u8 {
    let data: String = bits.take(3).collect();
    return u8::from_str_radix(&data, 2).unwrap();
}

fn parse_type_id(bits: &mut dyn Iterator<Item = char>) -> u8 {
    let data: String = bits.take(3).collect();
    return u8::from_str_radix(&data, 2).unwrap();
}

fn parse_literal(bits: &mut dyn Iterator<Item = char>) -> u64 {
    let mut lit_bits = Vec::new();
    loop {
        let last = bits.next().unwrap() == '0';
        lit_bits.extend(bits.take(4));
        if last {
            let bitstring = lit_bits.iter().collect::<String>();
            return u64::from_str_radix(&bitstring, 2).unwrap();
        }
    }
}

fn parse_binary_usize(bits: &mut dyn Iterator<Item = char>, nbits: usize) -> usize {
    usize::from_str_radix(&bits.take(nbits).collect::<String>(), 2).unwrap()
}

fn parse_operator_subpackets(bits: &mut dyn Iterator<Item = char>) -> Vec<Packet> {
    let mut res = Vec::new();

    let length_type_id = bits.next().unwrap();
    match length_type_id {
        '0' => {
            let nbits = parse_binary_usize(bits, 15);
            let mut sub_bits = bits.take(nbits).peekable();
            while sub_bits.peek().is_some() {
                res.push(parse_packet(&mut sub_bits));
            }
        }
        '1' => {
            for _ in 0..parse_binary_usize(bits, 11) {
                res.push(parse_packet(bits));
            }
        }
        _ => unreachable!(),
    }

    return res;
}

fn parse_packet(bits: &mut dyn Iterator<Item = char>) -> Packet {
    let version = parse_version(bits);
    let type_id = parse_type_id(bits);

    match type_id {
        4 => Packet::Literal(LiteralPacket {
            version,
            val: parse_literal(bits),
        }),
        _ => Packet::Operator(OperatorPacket {
            version,
            optype: match type_id {
                0 => OpType::Sum,
                1 => OpType::Product,
                2 => OpType::Min,
                3 => OpType::Max,
                5 => OpType::GreaterThan,
                6 => OpType::LessThan,
                7 => OpType::EqualTo,
                _ => unimplemented!(),
            },
            subpackets: parse_operator_subpackets(bits),
        }),
    }
}

fn eval(p: &Packet) -> u64 {
    match p {
        Packet::Literal(lit) => lit.val,
        Packet::Operator(op) => {
            let mut sub_vals = op.subpackets.iter().map(|sub| eval(sub));
            match op.optype {
                OpType::Sum => sub_vals.sum(),
                OpType::Product => sub_vals.product(),
                OpType::Min => sub_vals.min().unwrap(),
                OpType::Max => sub_vals.max().unwrap(),
                OpType::GreaterThan => {
                    if sub_vals.next().unwrap() > sub_vals.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                OpType::LessThan => {
                    if sub_vals.next().unwrap() < sub_vals.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                OpType::EqualTo => {
                    if sub_vals.next().unwrap() == sub_vals.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
            }
        }
    }
}

fn main() {
    let input = get_input();
    let mut bits = input
        .chars()
        .flat_map(|char| {
            let val = char.to_digit(16).unwrap();
            vec![
                val & 0b1000u32 != 0,
                val & 0b0100u32 != 0,
                val & 0b0010u32 != 0,
                val & 0b0001u32 != 0,
            ]
        })
        .map(|v| if v { '1' } else { '0' });

    println!("{}", eval(&parse_packet(&mut bits)));
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
