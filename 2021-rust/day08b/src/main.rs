use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
    str::FromStr,
};

// 0: 6 signals
// 1: 2 signals
// 2: 5 signals
// 3: 5 signals
// 4: 4 signals
// 5: 5 signals
// 6: 6 signals
// 7: 3 signals
// 8: 7 signals
// 9: 6 signals

#[derive(Debug)]
struct Sample {
    patterns: Vec<String>,
    output: Vec<String>,
}

impl FromStr for Sample {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks = s.split(" | ");
        return Ok(Sample {
            patterns: chunks
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| {
                    let mut chars = s.chars().collect::<Vec<char>>();
                    chars.sort();
                    chars.iter().collect()
                })
                .collect(),
            output: chunks
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| {
                    let mut chars = s.chars().collect::<Vec<char>>();
                    chars.sort();
                    chars.iter().collect()
                })
                .collect(),
        });
    }
}

fn main() {
    let samples: Vec<Sample> = get_input()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut result = 0;

    for s in samples.iter() {
        let mut string_to_digit: HashMap<String, i64> = HashMap::new();

        // Handle patterns with unique segment counts.
        for p in s.patterns.iter() {
            match p.len() {
                2 => {
                    string_to_digit.insert(p.clone(), 1);
                }
                3 => {
                    string_to_digit.insert(p.clone(), 7);
                }
                4 => {
                    string_to_digit.insert(p.clone(), 4);
                }
                7 => {
                    string_to_digit.insert(p.clone(), 8);
                }
                _ => (),
            };
        }

        assert!(string_to_digit.len() == 4);

        // Do frequency analysis.
        // Segment B appears 6 times.
        // Segment E appears 4 times.
        let mut segment_b_char = 'z';
        let mut segment_e_char = 'z';

        for c in 'a'..='g' {
            match s.patterns.iter().filter(|p| p.contains(c)).count() {
                6 => {
                    segment_b_char = c;
                    continue;
                }
                4 => {
                    segment_e_char = c;
                    continue;
                }
                _ => (),
            };
        }

        assert!(segment_b_char != 'z');
        assert!(segment_e_char != 'z');

        let five_segment_patterns: HashSet<String> = s
            .patterns
            .iter()
            .filter(|s| s.len() == 5)
            .map(|s| s.clone())
            .collect();
        assert!(five_segment_patterns.len() == 3);

        let mut used_five_segment_patterns: HashSet<String> = HashSet::new();
        for p in five_segment_patterns.iter() {
            // - The 5-segment pattern with the character for segment B is digit 5
            if p.contains(segment_b_char) {
                string_to_digit.insert(p.clone(), 5);
                used_five_segment_patterns.insert(p.clone());
                continue;
            }

            // - The 5-segment pattern with the character for segment E is digit 2
            if p.contains(segment_e_char) {
                string_to_digit.insert(p.clone(), 2);
                used_five_segment_patterns.insert(p.clone());
                continue;
            }
        }

        assert!(used_five_segment_patterns.len() == 2);

        // - The remaining unknown 5-segment pattern is digit 3.
        string_to_digit.insert(
            five_segment_patterns
                .difference(&used_five_segment_patterns)
                .next()
                .unwrap()
                .clone(),
            3,
        );

        let six_segment_patterns: HashSet<String> = s
            .patterns
            .iter()
            .filter(|s| s.len() == 6)
            .map(|s| s.clone())
            .collect();
        assert!(six_segment_patterns.len() == 3);

        let mut used_six_segment_patterns: HashSet<String> = HashSet::new();
        for p in six_segment_patterns.iter() {
            // - The 6-segment pattern WITHOUT the character for segment E is digit 9
            if !p.contains(segment_e_char) {
                string_to_digit.insert(p.clone(), 9);
                used_six_segment_patterns.insert(p.clone());
                continue;
            }

            // - The 6-segment pattern that is NOT digit 9 and has the 2 characters from the digit 1 is 0.
            let one_chars: Vec<char> = s
                .patterns
                .iter()
                .find(|p| p.len() == 2)
                .unwrap()
                .chars()
                .collect();

            if p.contains(one_chars[0]) && p.contains(one_chars[1]) {
                string_to_digit.insert(p.clone(), 0);
                used_six_segment_patterns.insert(p.clone());
                continue;
            }
        }

        assert!(used_six_segment_patterns.len() == 2);

        // - The remaining unknown 6-segment pattern is 6.
        string_to_digit.insert(
            six_segment_patterns
                .difference(&used_six_segment_patterns)
                .next()
                .unwrap()
                .clone(),
            6,
        );

        assert!(string_to_digit.len() == 10);
        result += string_to_digit[&s.output[0]] * 1000
            + string_to_digit[&s.output[1]] * 100
            + string_to_digit[&s.output[2]] * 10
            + string_to_digit[&s.output[3]] * 1;
    }

    println!("{}", result);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
