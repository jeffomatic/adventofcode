use std::{
    collections::HashMap,
    io::{self, Read},
};

fn num_sequences(src: &Vec<i64>, start: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if start == src.len() - 1 {
        return 1;
    }

    if let Some(&m) = memo.get(&start) {
        return m;
    }

    let mut candidates = Vec::new();
    for i in (start + 1)..src.len() {
        if src[i] - src[start] > 3 {
            break;
        }
        candidates.push(i);
    }

    let res = candidates
        .iter()
        .fold(0, |accum, &i| accum + num_sequences(src, i, memo));
    memo.insert(start, res);
    res
}

fn main() {
    let mut adapters: Vec<i64> = vec![0];
    for line in get_input().lines() {
        adapters.push(line.parse().unwrap());
    }
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);

    let mut memo = HashMap::new();
    let res = num_sequences(&adapters, 0, &mut memo);
    println!("{}", res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
