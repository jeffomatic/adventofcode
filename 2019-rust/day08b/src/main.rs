use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let input = get_input();
    let data: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let w = 25;
    let h = 6;
    let layers: Vec<Vec<u32>> = data.chunks(w * h).map(|v| v.to_owned()).collect();

    let mut img = Vec::new();
    for i in 0..(h * w) {
        for lyr in layers.iter() {
            if lyr[i] != 2 {
                img.push(lyr[i]);
                break;
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            match img[(i * w) + j] {
                0 => print!(" "),
                1 => print!("*"),
                2 => print!("T"),
                _ => panic!("invalid pixel ({}, {})", i, j),
            }
        }
        print!("\n");
    }
}
