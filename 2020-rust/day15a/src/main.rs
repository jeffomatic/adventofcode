use std::collections::HashMap;

fn main() {
    let input = "5,2,8,16,18,0,1";
    let mut turn = 1;
    let mut history: HashMap<i64, i64> = HashMap::new();
    let mut prev = -1;
    for n in input.split(",").map(|c| c.parse::<i64>().unwrap()) {
        if prev != -1 {
            history.insert(prev, turn - 1);
        }

        prev = n;
        turn += 1;
    }

    loop {
        let n = if let Some(past_turn) = history.get(&prev) {
            turn - past_turn - 1
        } else {
            0
        };

        history.insert(prev, turn - 1);

        if turn == 2020 {
            println!("{}", n);
            break;
        }

        prev = n;
        turn += 1;
    }
}
