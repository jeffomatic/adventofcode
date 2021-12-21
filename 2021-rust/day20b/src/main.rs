use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() {
    let input = get_input();
    let mut lines = input.lines();
    let lookup: Vec<char> = lines.next().unwrap().chars().collect();

    // consume newline
    lines.next();

    let mut image: HashSet<(i64, i64)> = HashSet::new();
    let mut rmin = i64::MAX;
    let mut rmax = i64::MIN;
    let mut cmin = i64::MAX;
    let mut cmax = i64::MIN;
    for (row, line) in lines.enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                rmin = rmin.min(row as i64);
                rmax = rmax.max(row as i64);
                cmin = cmin.min(col as i64);
                cmax = cmax.max(col as i64);

                image.insert((row as i64, col as i64));
            }
        }
    }

    for step in 0..50 {
        let (exists_val, not_exists_val) = if step % 2 == 0 {
            ('1', '0')
        } else {
            ('0', '1')
        };

        let mut next_image: HashSet<(i64, i64)> = HashSet::new();
        let mut next_rmin = i64::MAX;
        let mut next_rmax = i64::MIN;
        let mut next_cmin = i64::MAX;
        let mut next_cmax = i64::MIN;

        for out_row in (rmin - 1)..=(rmax + 1) {
            for out_col in (cmin - 1)..=(cmax + 1) {
                let mut digits: Vec<char> = Vec::new();
                for i in (out_row - 1)..=(out_row + 1) {
                    for j in (out_col - 1)..=(out_col + 1) {
                        if image.contains(&(i, j)) {
                            digits.push(exists_val);
                        } else {
                            digits.push(not_exists_val);
                        }
                    }
                }

                let bin_str: String = digits.iter().collect();
                let index = usize::from_str_radix(&bin_str, 2).unwrap();

                let c = if step % 2 == 0 { '.' } else { '#' };
                if lookup[index] == c {
                    next_image.insert((out_row, out_col));

                    next_rmin = next_rmin.min(out_row as i64);
                    next_rmax = next_rmax.max(out_row as i64);
                    next_cmin = next_cmin.min(out_col as i64);
                    next_cmax = next_cmax.max(out_col as i64);
                }
            }
        }

        image = next_image;
        rmin = next_rmin;
        rmax = next_rmax;
        cmin = next_cmin;
        cmax = next_cmax;
    }

    println!("{}", image.len());
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
