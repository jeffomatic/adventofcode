use std::io::{self, Read};

fn main() {
    let mut state = [0 as u64; 9];
    for n in get_input()
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
    {
        state[n] += 1;
    }

    let num_cycles = 80;
    for c in 0..num_cycles {
        let num_spawning = state[0];
        for i in 0..=7 {
            state[i] = state[i + 1];
        }
        state[6] += num_spawning; // newly spawned fish
        state[8] = num_spawning; // fish that just spawned
        dbg!(c, state);
    }

    println!("{}", state.iter().fold(0, |accum, n| accum + n));
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}

// Initial state: 3,4,3,1,2
// After  1 day:  2,3,2,0,1
// After  2 days: 1,2,1,6,0,8
// After  3 days: 0,1,0,5,6,7,8
// After  4 days: 6,0,6,4,5,6,7,8,8
// After  5 days: 5,6,5,3,4,5,6,7,7,8
// After  6 days: 4,5,4,2,3,4,5,6,6,7
// After  7 days: 3,4,3,1,2,3,4,5,5,6
// After  8 days: 2,3,2,0,1,2,3,4,4,5
// After  9 days: 1,2,1,6,0,1,2,3,3,4,8
// After 10 days: 0,1,0,5,6,0,1,2,2,3,7,8
// After 11 days: 6,0,6,4,5,6,0,1,1,2,6,7,8,8,8
// After 12 days: 5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8
// After 13 days: 4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8
// After 14 days: 3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8
// After 15 days: 2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7
// After 16 days: 1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8
// After 17 days: 0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8
// After 18 days: 6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8
