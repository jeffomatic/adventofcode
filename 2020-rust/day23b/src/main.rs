fn main() {
    let maxval = 1_000_000;
    let num_rounds = 10_000_000;
    let mut nexts = vec![0; maxval + 1];

    // insert initial values
    let input = "562893147";
    let mut initial_vals = input.chars().map(|c| c.to_digit(10).unwrap() as usize);
    let first = initial_vals.next().unwrap();

    let mut cur = first;
    for next in initial_vals {
        nexts[cur] = next;
        cur = next;
    }

    // backfill the remaining values
    for next in (input.len() + 1)..=maxval {
        nexts[cur] = next;
        cur = next;
    }

    // complete the ring
    nexts[cur] = first;

    // play the game
    cur = first;
    for _ in 0..num_rounds {
        let move1 = nexts[cur];
        let move2 = nexts[move1];
        let move3 = nexts[move2];

        // remove the above 3 items from the list
        nexts[cur] = nexts[move3];

        // calculate destination
        let mut dest = cur;
        while dest == cur || dest == move1 || dest == move2 || dest == move3 {
            dest -= 1;
            if dest == 0 {
                dest = maxval;
            }
        }

        // re-insert items into the list
        let temp = nexts[dest];
        nexts[dest] = move1;
        nexts[move3] = temp;

        // advance to next item
        cur = nexts[cur];
    }

    println!("{}", nexts[1] * nexts[nexts[1]]);
}
