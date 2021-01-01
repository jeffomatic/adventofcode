use regex::Regex;
use std::io::{self, Read};

fn main() {
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    let byr = Regex::new(r#"byr:(\d{4})\b"#).unwrap();

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    let iyr = Regex::new(r#"iyr:(\d{4})\b"#).unwrap();

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    let eyr = Regex::new(r#"eyr:(\d{4})\b"#).unwrap();

    // hgt (Height) - a number followed by either cm or in:
    //     If cm, the number must be at least 150 and at most 193.
    //     If in, the number must be at least 59 and at most 76.
    let hgt = Regex::new(r#"hgt:(\d+)(cm|in)\b"#).unwrap();

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    let hcl = Regex::new(r#"hcl:#[0-9a-f]{6}\b"#).unwrap();

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    let ecl = Regex::new(r#"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b"#).unwrap();

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    let pid = Regex::new(r#"pid:[0-9]{9}\b"#).unwrap();

    let n = get_input()
        .split("\n\n")
        .filter(|&p| {
            // byr (Birth Year) - four digits; at least 1920 and at most 2002.
            match byr.captures(p) {
                None => return false,
                Some(caps) => {
                    let v = caps[1].parse::<i32>().unwrap();
                    if v < 1920 || 2002 < v {
                        return false;
                    };
                }
            };

            // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            match iyr.captures(p) {
                None => return false,
                Some(caps) => {
                    let v = caps[1].parse::<i32>().unwrap();
                    if v < 2010 || 2020 < v {
                        return false;
                    };
                }
            };

            // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            match eyr.captures(p) {
                None => return false,
                Some(caps) => {
                    let v = caps[1].parse::<i32>().unwrap();
                    if v < 2020 || 2030 < v {
                        return false;
                    };
                }
            };

            // hgt (Height) - a number followed by either cm or in:
            //     If cm, the number must be at least 150 and at most 193.
            //     If in, the number must be at least 59 and at most 76.
            match hgt.captures(p) {
                None => return false,
                Some(caps) => {
                    let v = caps[1].parse::<i32>().unwrap();
                    match &caps[2] {
                        "cm" => {
                            if v < 150 || 193 < v {
                                return false;
                            }
                        }
                        "in" => {
                            if v < 59 || 76 < v {
                                return false;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            };

            // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            // pid (Passport ID) - a nine-digit number, including leading zeroes.
            hcl.is_match(p) && ecl.is_match(p) && pid.is_match(p)
        })
        .count();
    println!("{}", n);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
