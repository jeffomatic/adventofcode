use intcode;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let program: Vec<i64> = intcode::program_from_string(&input);

    let mut cpus: Vec<intcode::Computer> =
        (0..50).map(|_| intcode::Computer::new(&program)).collect();
    let mut in_queues: Vec<Vec<i64>> = (0..50).map(|i| vec![i]).collect();

    let default_input = vec![-1];
    let mut nat = Vec::new();
    let mut prev_nat_y = None;

    loop {
        for (src, cpu) in cpus.iter_mut().enumerate() {
            let input = if in_queues[src].is_empty() {
                &default_input
            } else {
                &in_queues[src]
            };

            let result = cpu.run(input);
            in_queues[src] = result.unused_input;

            for chunk in result.output.chunks(3) {
                let dst = chunk[0] as usize;
                if dst == 255 {
                    nat = vec![chunk[1], chunk[2]];
                    continue;
                }

                in_queues[dst].push(chunk[1]);
                in_queues[dst].push(chunk[2]);
            }
        }

        if !in_queues.iter().any(|q| !q.is_empty()) {
            if let Some(prev) = prev_nat_y {
                if prev == nat[1] {
                    println!("{}", prev);
                    return;
                }
            }

            if !nat.is_empty() {
                in_queues[0] = nat.to_vec();
                prev_nat_y = Some(nat[1]);
            }
        }
    }
}
