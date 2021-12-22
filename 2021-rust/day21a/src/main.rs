use std::collections::HashMap;

fn main() {
    let mut rolls: Vec<i32> = Vec::new();
    let mut next = 1;
    loop {
        let a = next;
        let b = ((next + 1 - 1) % 100) + 1;
        let c = ((next + 2 - 1) % 100) + 1;
        rolls.push(a + b + c);
        next = ((next + 3 - 1) % 100) + 1;

        if next == 1 {
            break;
        }
    }

    let mut outcomes: HashMap<(i32, usize), i32> = HashMap::new();
    for pos in 1..=10 {
        for (i, steps) in rolls.iter().enumerate() {
            let next_pos = ((pos + steps - 1) % 10) + 1;
            outcomes.insert((pos, i), next_pos);
        }
    }

    let mut p1 = 8;
    let mut s1 = 0;
    let mut p2 = 1;
    let mut s2 = 0;
    let mut num_rolls = 0;
    let mut next_roll = 0;

    loop {
        let next_p1 = outcomes[&(p1, next_roll)];
        p1 = next_p1;
        s1 += next_p1;

        next_roll = (next_roll + 1) % rolls.len();
        num_rolls += 3;

        if s1 >= 1000 {
            break;
        }

        let next_p2 = outcomes[&(p2, next_roll)];
        p2 = next_p2;
        s2 += next_p2;

        next_roll = (next_roll + 1) % rolls.len();
        num_rolls += 3;

        if s2 >= 1000 {
            break;
        }
    }

    println!("{}", num_rolls * s1.min(s2));
}
