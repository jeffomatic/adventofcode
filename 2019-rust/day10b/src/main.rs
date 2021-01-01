use num::integer::gcd;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn delta(from: (usize, usize), to: (usize, usize)) -> (i64, i64) {
    (to.0 as i64 - from.0 as i64, to.1 as i64 - from.1 as i64)
}

// slope is rise / run, so the y-axis component is the first component
fn slope(from: (usize, usize), to: (usize, usize)) -> (i64, i64) {
    if from.0 == to.0 {
        panic!("infinite slope between {:?} and {:?}", from, to);
    }

    let delta = delta(from, to);
    let gcd = gcd(delta.0, delta.1);
    (delta.1 / gcd, delta.0 / gcd)
}

// sorts slopes from shallowest (negative) to steepest (positive)
fn cmp_slopes(a: (i64, i64), b: (i64, i64)) -> Ordering {
    (a.0 as f64 / a.1 as f64)
        .partial_cmp(&(b.0 as f64 / b.1 as f64))
        .unwrap_or(Ordering::Equal)
}

fn sorted_radial_slopes(dimensions: (usize, usize), center: (usize, usize)) -> Vec<(i64, i64)> {
    // start with up vector
    let mut sorted_slopes = vec![(-1, 0)];
    let mut dedupe = HashSet::new();

    // quadrants I and II
    let mut q1q2 = Vec::new();
    for i in 0..dimensions.1 {
        for j in (center.0 + 1)..dimensions.0 {
            let slope = slope(center, (j, i));
            if dedupe.contains(&slope) {
                continue;
            }

            q1q2.push(slope);
            dedupe.insert(slope);
        }
    }

    q1q2.sort_by(|a, b| cmp_slopes(*a, *b));
    sorted_slopes.append(&mut q1q2);

    // add down vector
    sorted_slopes.push((1, 0));

    // quadrants III and IV
    let mut q3q4 = Vec::new();
    for i in 0..dimensions.1 {
        for j in 0..center.0 {
            let slope = slope(center, (j, i));
            if dedupe.contains(&slope) {
                continue;
            }

            q3q4.push(slope);
            dedupe.insert(slope);
        }
    }

    q3q4.sort_by(|a, b| cmp_slopes(*a, *b));
    sorted_slopes.append(&mut q3q4);

    sorted_slopes
}

fn main() {
    let input = get_input();
    let map: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("invalid character"),
                })
                .collect()
        })
        .collect();

    let station = (19, 11);
    let scan_until = 200;
    let h = map.len();
    let w = map[0].len();
    let sorted_slopes = sorted_radial_slopes((w, h), station);
    let mut destroyed = HashSet::new();

    loop {
        for slope in sorted_slopes.iter() {
            // cast a ray using the current slope
            let mut n = 0;
            loop {
                n += 1;
                let disp = (slope.1 * n, slope.0 * n);
                let target = (station.0 as i64 + disp.0, station.1 as i64 + disp.1);

                // Ensure we haven't exceeded the map boundaries
                if target.0 < 0 || w as i64 <= target.0 || target.1 < 0 || h as i64 <= target.1 {
                    break; // end raycast and advance to next slope in the cycle
                }

                // See if there is an asteroid and check that it isn't already destroyed
                if map[target.1 as usize][target.0 as usize] && !destroyed.contains(&target) {
                    destroyed.insert(target);

                    // If we have hit the limit, we're done
                    if destroyed.len() == scan_until {
                        println!("{:?} {}", target, 100 * target.0 + target.1);
                        return;
                    }

                    break; // end raycast and advance to next slope in the cycle
                }
            }
        }
    }
}
