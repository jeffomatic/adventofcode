use std::io::{self, Read};

#[derive(Debug, Clone)]
struct LiteralPacket {
    version: u8,
    val: u64,
}

#[derive(Debug, Clone)]
struct OperatorPacket {
    version: u8,
    type_id: u8,
    subpackets: Vec<Packet>,
}

#[derive(Debug, Clone)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

fn read_version(bits: &mut dyn Iterator<Item = char>) -> u8 {
    let version_bits: String = bits.take(3).collect();
    return u8::from_str_radix(&version_bits, 2).unwrap();
}

fn read_type_id(bits: &mut dyn Iterator<Item = char>) -> u8 {
    let version_bits: String = bits.take(3).collect();
    return u8::from_str_radix(&version_bits, 2).unwrap();
}

fn read_literal(bits: &mut dyn Iterator<Item = char>) -> u64 {
    let mut literal_bits = Vec::new();
    loop {
        let last = bits.next().unwrap() == '0';
        literal_bits.extend(bits.take(4));
        if last {
            let bitstring = literal_bits.iter().collect::<String>();
            return u64::from_str_radix(&bitstring, 2).unwrap();
        }
    }
}

fn read_operator_subpackets(bits: &mut dyn Iterator<Item = char>) -> Vec<Packet> {
    let mut res = Vec::new();

    let length_type_id = bits.next().unwrap();
    match length_type_id {
        '0' => {
            let nbits = usize::from_str_radix(&bits.take(15).collect::<String>(), 2).unwrap();
            let mut subpacket_bits = bits.take(nbits).peekable();
            while let Some(_) = subpacket_bits.peek() {
                res.push(read_packet(&mut subpacket_bits));
            }
        }
        '1' => {
            let nsubpackets = usize::from_str_radix(&bits.take(11).collect::<String>(), 2).unwrap();
            for _ in 0..nsubpackets {
                res.push(read_packet(bits));
            }
        }
        _ => unreachable!(),
    }

    return res;
}

fn read_packet(bits: &mut dyn Iterator<Item = char>) -> Packet {
    let version = read_version(bits);
    let type_id = read_type_id(bits);

    match type_id {
        4 => Packet::Literal(LiteralPacket {
            version,
            val: read_literal(bits),
        }),
        _ => Packet::Operator(OperatorPacket {
            version,
            type_id,
            subpackets: read_operator_subpackets(bits),
        }),
    }
}

fn eval(p: &Packet) -> u64 {
    match p {
        Packet::Literal(lit) => lit.val,
        Packet::Operator(op) => {
            let mut sub_vals = op.subpackets.iter().map(|sub| eval(sub));
            match op.type_id {
                0 => sub_vals.sum(),
                1 => sub_vals.product(),
                2 => sub_vals.min().unwrap(),
                3 => sub_vals.max().unwrap(),
                5 => {
                    if sub_vals.next().unwrap() > sub_vals.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if sub_vals.next().unwrap() < sub_vals.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if sub_vals.next().unwrap() == sub_vals.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
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

    println!("{}", eval(&read_packet(&mut bits)));
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
