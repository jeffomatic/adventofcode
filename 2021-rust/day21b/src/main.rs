use std::collections::HashMap;

// 3d3 dice roll outcomes:
// 3 4 4 5 5 5 6 6 7
// 4 5 5 6 6 6 7 7 8
// 5 6 6 7 7 7 8 8 9
const HISTOGRAM: [(u8, u8); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    p1_pos: u8,
    p1_score: u8,
    p2_pos: u8,
    p2_score: u8,
    p1_turn: bool,
}

fn count_wins(state: State, cache: &mut HashMap<State, (i64, i64)>) -> (i64, i64) {
    if state.p1_score >= 21 {
        return (1, 0);
    }

    if state.p2_score >= 21 {
        return (0, 1);
    }

    let mut total = (0, 0);
    for &(steps, freq) in HISTOGRAM.iter() {
        let mut next_state = state;
        if state.p1_turn {
            next_state.p1_pos = (state.p1_pos + steps - 1) % 10 + 1;
            next_state.p1_score += next_state.p1_pos;
            next_state.p1_turn = false;
        } else {
            next_state.p2_pos = (state.p2_pos + steps - 1) % 10 + 1;
            next_state.p2_score += next_state.p2_pos;
            next_state.p1_turn = true;
        }

        let (p1_wins, p2_wins) = match cache.get(&next_state) {
            Some(&res) => res,
            None => {
                let res = count_wins(next_state, cache);
                cache.insert(next_state, res);
                res
            }
        };

        total.0 += p1_wins * freq as i64;
        total.1 += p2_wins * freq as i64;
    }

    total
}

fn main() {
    let (p1_wins, p2_wins) = count_wins(
        State {
            p1_pos: 8,
            p1_score: 0,
            p2_pos: 1,
            p2_score: 0,
            p1_turn: true,
        },
        &mut HashMap::new(),
    );
    println!("p1 {} p2 {}: {}", p1_wins, p2_wins, p1_wins.max(p2_wins));
}
