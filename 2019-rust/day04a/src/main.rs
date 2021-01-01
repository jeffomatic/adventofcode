fn digits(mut n: i64) -> Vec<i64> {
    let mut res = Vec::new();
    loop {
        if n < 10 {
            res.push(n);
            break;
        }
        res.push(n % 10);
        n = n / 10;
    }
    res.reverse();
    return res;
}

fn increasing(v: &Vec<i64>) -> bool {
    let mut last = -1;
    for n in v.iter() {
        if *n < last {
            return false;
        }
        last = *n;
    }
    return true;
}

fn has_adjacent(v: &Vec<i64>) -> bool {
    let mut last = -1;
    for n in v.iter() {
        if *n == last {
            return true;
        }
        last = *n;
    }
    return false;
}

fn main() {
    let start = 134792;
    let end = 675810;
    println!(
        "{}",
        (start..=end)
            .map(digits)
            .filter(increasing)
            .filter(has_adjacent)
            .count()
    );
}
