use std::io::{self, Read};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Mapping {
    src_start: usize,
    dst_start: usize,
    range: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Data {
    seeds: Vec<usize>,
    maps: Vec<Vec<Mapping>>,
}

fn parse_mapping(line: &str) -> Mapping {
    let vals: Vec<usize> = line.split(" ").map(|s| s.parse().unwrap()).collect();
    Mapping {
        src_start: vals[1],
        dst_start: vals[0],
        range: vals[2],
    }
}

fn parse_map(chunk: &str) -> Vec<Mapping> {
    let mut res: Vec<Mapping> = chunk
        .trim()
        .split("\n")
        .skip(1)
        .map(parse_mapping)
        .collect();
    res.sort_by(|a, b| a.src_start.cmp(&b.src_start));
    res
}

fn parse(input: &str) -> Data {
    let chunks: Vec<&str> = input.trim().split("\n\n").collect();
    let seeds: Vec<usize> = chunks[0]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect();
    let maps: Vec<Vec<Mapping>> = chunks.iter().skip(1).map(|&s| parse_map(s)).collect();
    Data { seeds, maps }
}

fn resolve(seed: usize, maps: &Vec<Vec<Mapping>>) -> usize {
    let mut next = seed;
    for map in maps {
        for m in map {
            if next < m.src_start {
                break;
            }
            if next < m.src_start + m.range {
                next = m.dst_start + (next - m.src_start);
                break;
            }
        }
    }
    next
}

fn main() {
    let input = get_input();
    let data = parse(&input);
    let res = data
        .seeds
        .iter()
        .map(|&s| resolve(s, &data.maps))
        .min()
        .unwrap();
    println!("{}", res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
