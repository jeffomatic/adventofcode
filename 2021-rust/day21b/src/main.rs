use std::collections::HashMap;

// How many ways can player one get to 21 or greater
// dice roll table:
// 3 4 4 5 5 5 6 6 7
// 4 5 5 6 6 6 7 7 8
// 5 6 6 7 7 7 8 8 9
//
// Histogram:
// 3: 1
// 4: 3
// 5: 6
// 6: 7
// 7: 6
// 8: 3
// 9: 1

const HISTOGRAM: [(u8, u8); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

struct Solver {
    cache: HashMap<(u8, u8, u8, u8, bool), (i64, i64)>,
}

impl Solver {
    fn count_wins(
        &mut self,
        p1_pos: u8,
        p1_score: u8,
        p2_pos: u8,
        p2_score: u8,
        p1_turn: bool,
    ) -> (i64, i64) {
        if p1_score >= 21 {
            return (1, 0);
        }

        if p2_score >= 21 {
            return (0, 1);
        }

        let mut total = (0, 0);
        for &(steps, freq) in HISTOGRAM.iter() {
            let next_args = if p1_turn {
                let next_p1_pos = (p1_pos + steps - 1) % 10 + 1;
                let next_p1_score = p1_score + next_p1_pos;
                (next_p1_pos, next_p1_score, p2_pos, p2_score, false)
            } else {
                let next_p2_pos = (p2_pos + steps - 1) % 10 + 1;
                let next_p2_score = p2_score + next_p2_pos;
                (p1_pos, p1_score, next_p2_pos, next_p2_score, true)
            };

            let (p1_wins, p2_wins) = match self.cache.get(&next_args) {
                Some(&res) => res,
                None => {
                    let res = self.count_wins(
                        next_args.0,
                        next_args.1,
                        next_args.2,
                        next_args.3,
                        next_args.4,
                    );
                    self.cache.insert(next_args, res);
                    res
                }
            };

            total.0 += p1_wins * freq as i64;
            total.1 += p2_wins * freq as i64;
        }

        total
    }
}

fn main() {
    let mut solver = Solver {
        cache: HashMap::new(),
    };

    let (p1_wins, p2_wins) = solver.count_wins(8, 0, 1, 0, true);
    println!("p1 {} p2 {}: {}", p1_wins, p2_wins, p1_wins.max(p2_wins));
}
