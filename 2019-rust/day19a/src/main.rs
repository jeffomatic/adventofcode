use intcode;
use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let program: Vec<i64> = get_input().split(",").map(|s| s.parse().unwrap()).collect();
    let mut area = 0;

    for y in 0..50 {
        for x in 0..50 {
            let result = intcode::Computer::new(&program).run(&vec![x, y]);
            let affected = result.output[0] == 1;
            if affected {
                area += 1;
            }
            print!("{}", if affected { "#" } else { "." });
        }
        print!("\n");
    }

    println!("affected area: {}", area);
}
