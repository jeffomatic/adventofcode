use intcode;
use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut program: Vec<i64> = get_input().split(",").map(|s| s.parse().unwrap()).collect();

    // play for free; set address 0 to 2
    program[0] = 2;

    let mut cpu = intcode::Computer::new(&program);
    let mut input = Vec::new();
    let mut output_log = Vec::new();

    let mut score = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;

    loop {
        let mut result = cpu.run(&input);

        input = result.unused_input;
        if !input.is_empty() {
            panic!("input should have been fully consumed");
        }

        // TODO: maintain block state so we don't have to replay the whole log
        output_log.append(&mut result.output);

        let mut nblocks = 0;
        for d in output_log.chunks(3) {
            if d[0] == -1 && d[1] == 0 {
                score = d[2];
                continue;
            }

            let x = d[0] as usize;

            // 0 is an empty tile. No game object appears in this tile.
            // 1 is a wall tile. Walls are indestructible barriers.
            // 2 is a block tile. Blocks can be broken by the ball.
            // 3 is a horizontal paddle tile. The paddle is indestructible.
            // 4 is a ball tile. The ball moves diagonally and bounces off objects.
            match d[2] {
                0 => (),
                1 => (),
                2 => nblocks += 1,
                3 => paddle_x = x,
                4 => ball_x = x,
                t => panic!("invalid tile: {}", t),
            }
        }

        match result.state {
            intcode::State::BlockedOnRead => {
                if paddle_x < ball_x {
                    input.push(1);
                } else if ball_x < paddle_x {
                    input.push(-1);
                } else {
                    input.push(0);
                }
            }
            intcode::State::Halted => break,
        }

        if nblocks == 0 {
            break;
        }
    }

    println!("{}", score);
}
