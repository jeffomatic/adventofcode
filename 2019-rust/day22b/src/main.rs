use std::io::{self, Read};

mod modulo {
    pub fn normalize(n: i128, modulus: i128) -> i128 {
        if n < 0 {
            modulus - (-n % modulus)
        } else {
            n % modulus
        }
    }

    pub fn divide(mut n: i128, d: i128, modulus: i128) -> i128 {
        while n % d != 0 {
            n += modulus;
        }
        normalize(n / d, modulus)
    }
}

mod transform {
    use crate::modulo;

    // A linear transform, of the form (ax + b). The first component is the
    // coefficient, and the second component is the offset.
    pub type Transform = (i128, i128);

    pub fn compose(inner: Transform, outer: Transform, modulus: i128) -> Transform {
        (
            modulo::normalize(outer.0 * inner.0, modulus),
            modulo::normalize((outer.0 * inner.1) + outer.1, modulus),
        )
    }

    pub fn power_compose(tf: Transform, count: usize, modulus: i128) -> Transform {
        if count == 0 {
            return (1, 0);
        }

        let mut next = tf;
        let mut applications = 1;
        while applications * 2 <= count {
            next = compose(next, next, modulus);
            applications *= 2;
        }

        compose(
            next,
            power_compose(tf, count - applications, modulus),
            modulus,
        )
    }

    pub fn apply(transform: Transform, x: i128, modulus: i128) -> i128 {
        modulo::normalize(transform.0 * x + transform.1, modulus)
    }

    // ax + b = y mod m
    // ax = y - b mod m
    // x = y/a - b/a mod m
    pub fn invert(transform: Transform, modulus: i128) -> Transform {
        (
            modulo::normalize(modulo::divide(1, transform.0, modulus), modulus),
            modulo::normalize(modulo::divide(-transform.1, transform.0, modulus), modulus),
        )
    }
}

mod shuffle {
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::str::FromStr;

    use crate::transform;

    #[derive(Clone, Copy, Debug)]
    pub enum Command {
        Cut(i128),
        DealNew,
        DealWithIncrement(i128),
    }

    impl FromStr for Command {
        type Err = Box<dyn std::error::Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            lazy_static! {
                static ref RE: Regex =
                    Regex::new(r"(?P<command>[A-Za-z ]+)(?P<arg>-?\d+)?").unwrap();
            }

            let caps = match RE.captures(s) {
                None => return Err(From::from("invalid command format")),
                Some(caps) => caps,
            };

            match caps["command"].parse::<String>().unwrap().as_str() {
                "cut " => Ok(Command::Cut(caps["arg"].parse().unwrap())),
                "deal into new stack" => Ok(Command::DealNew),
                "deal with increment " => {
                    Ok(Command::DealWithIncrement(caps["arg"].parse().unwrap()))
                }
                _ => return Err(From::from("unrecognized command")),
            }
        }
    }

    // Each shuffle command is a linear function mapping the current position of
    // a card to its new position. The mapping can be applied uniformly to every
    // card in the deck.
    pub fn to_transform(c: &Command, len: i128) -> transform::Transform {
        match *c {
            Command::Cut(amount) => (1, -amount),
            Command::DealNew => (-1, len - 1),
            Command::DealWithIncrement(increment) => (increment, 0),
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let commands: Vec<shuffle::Command> = input
        .trim()
        .to_string()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let len = 119315717514047;
    let iterations = 101741582076661;
    let target_pos = 2020;

    // inverting the power composition of the prev-to-next transform is too
    // costly, so we will incrementally invert in reverse order, and
    // power-compose the result.
    let inverse = commands.iter().rev().fold((1, 0), |acc, c| {
        transform::compose(
            acc,
            transform::invert(shuffle::to_transform(c, len), len),
            len,
        )
    });
    println!("next to prev: {:?}", inverse);

    let power_inverse = transform::power_compose(inverse, iterations, len);
    println!("next to prev {} times: {:?}", iterations, power_inverse);

    println!("{}", transform::apply(power_inverse, target_pos, len));
}
