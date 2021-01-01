fn main() {
    let input = "5,2,8,16,18,0,1";
    let mut turn = 1;
    let mut history: Vec<i32> = vec![-1; 30_000_000];
    let mut prev: usize = usize::MAX;
    for n in input.split(",").map(|c| c.parse::<i64>().unwrap()) {
        if prev != usize::MAX {
            history[prev] = turn - 1;
        }

        prev = n as usize;
        turn += 1;
    }

    loop {
        let past_turn = history[prev];
        let n = if past_turn == -1 {
            0
        } else {
            turn - past_turn - 1
        };

        history[prev] = turn - 1;

        if turn == 30_000_000 {
            println!("{}", n);
            break;
        }

        prev = n as usize;
        turn += 1;
    }
}
