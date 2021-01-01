use std::io::{self, Read};

fn main() {
    let n = get_input()
        .split("\n\n")
        .filter(|&p| {
            p.find("byr").is_some()
                && p.find("iyr").is_some()
                && p.find("eyr").is_some()
                && p.find("hgt").is_some()
                && p.find("hcl").is_some()
                && p.find("ecl").is_some()
                && p.find("pid").is_some()
        })
        .count();
    println!("{}", n);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
