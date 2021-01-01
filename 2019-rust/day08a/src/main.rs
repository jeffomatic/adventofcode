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

    let mut min_zeroes = 1000000;
    let mut min_layer = &layers[0];
    for lyr in layers.iter() {
        let nzeros = lyr.iter().filter(|d| **d == 0).count();
        if nzeros < min_zeroes {
            min_zeroes = nzeros;
            min_layer = &lyr;
        }
    }

    println!(
        "{}",
        min_layer.iter().filter(|d| **d == 1).count()
            * min_layer.iter().filter(|d| **d == 2).count()
    );
}
