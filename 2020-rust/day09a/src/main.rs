use std::{
    collections::VecDeque,
    io::{self, Read},
};

fn main() {
    let mut vals: VecDeque<i64> = VecDeque::new();
    let mut preamble = true;
    for n in get_input().lines().map(|line| line.parse::<i64>().unwrap()) {
        if preamble {
            vals.push_back(n);

            if vals.len() < 25 {
                continue;
            } else {
                preamble = false;
            }
        }

        let mut ok = false;
        'outer: for i in 0..vals.len() {
            for j in i..vals.len() {
                if vals[i] + vals[j] == n {
                    ok = true;
                    vals.pop_front();
                    vals.push_back(n);
                    break 'outer;
                }
            }
        }

        if !ok {
            println!("{}", n);
            return;
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
