use std::io::{self, Read};

fn get_input() -> String {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();
    handle.read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}

fn main() {
    let max_dist = 10000 - 1;
    let nodes: Vec<_> = get_input()
        .lines()
        .map(|s| s.split(", "))
        .map(|mut toks| {
            (
                toks.next().unwrap().parse::<i32>().unwrap(),
                toks.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    let mut size = 0;
    'rows: for i in -1000..=1000 {
        'columns: for j in -1000..=1000 {
            let mut dist_sum = 0;
            for n in nodes.iter() {
                dist_sum += (n.0 - j).abs() + (n.1 - i).abs();
                if dist_sum > max_dist {
                    continue 'columns;
                }
            }
            size += 1;
        }
    }

    println!("{}", size);
}
