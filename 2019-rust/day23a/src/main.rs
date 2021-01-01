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
                    println!("{}", chunk[2]);
                    return;
                }

                in_queues[dst].push(chunk[1]);
                in_queues[dst].push(chunk[2]);
            }
        }
    }
}
