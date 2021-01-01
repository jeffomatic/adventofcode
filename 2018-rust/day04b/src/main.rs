use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Timestamp {
    y: i32,
    month: i32,
    d: i32,
    h: i32,
    min: i32,
}

#[derive(Debug)]
enum LogEvent {
    Start(i32),
    Sleep,
    Wake,
}

#[derive(Debug)]
struct Log {
    ts: Timestamp,
    event: LogEvent,
}

impl FromStr for Log {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Log, Self::Err> {
        lazy_static! {
          /*
          [1518-05-24 23:56] Guard #1721 begins shift
          [1518-08-22 00:09] falls asleep
          [1518-05-19 00:53] wakes up
          */
          static ref RE: Regex = Regex::new(r"(?x)
              \[
                  (?P<y>\d{4})-(?P<month>\d{2})-(?P<d>\d{2})
                  \s
                  (?P<h>\d{2}):(?P<min>\d{2})
              \]
              \s
              (?P<event>(
                  Guard\ \#(?P<guard_id>\d+)\ begins\ shift
                  |
                  wakes\ up
                  |
                  falls\ asleep
              ))
          ").unwrap();
        }

        let caps = match RE.captures(s) {
            None => return Err(From::from("invalid log entry")),
            Some(caps) => caps,
        };

        return Ok(Log {
            ts: Timestamp {
                y: caps["y"].parse()?,
                month: caps["month"].parse()?,
                d: caps["d"].parse()?,
                h: caps["h"].parse()?,
                min: caps["min"].parse()?,
            },
            event: match &caps["event"] {
                "wakes up" => LogEvent::Wake,
                "falls asleep" => LogEvent::Sleep,
                _ => LogEvent::Start(caps["guard_id"].parse()?),
            },
        });
    }
}

fn mode<T>(items: impl std::iter::Iterator<Item = T>) -> Option<(T, i32)>
where
    T: std::hash::Hash,
    T: Eq,
    T: Copy,
{
    let mut freqs = HashMap::new();
    let mut max_f = 0;
    let mut max_i = None;

    for i in items {
        let f = 1 + match freqs.get(&i) {
            Some(f) => *f,
            None => 0,
        };

        freqs.insert(i, f);

        if f > max_f {
            max_f = f;
            max_i = Some(i.clone());
        }
    }

    match max_i {
        Some(i) => Some((i, max_f)),
        None => None,
    }
}

fn get_input() -> String {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();
    handle.read_to_string(&mut input).unwrap();
    return input;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut logs: Vec<_> = get_input()
        .lines()
        .map(|s| s.parse::<Log>().unwrap())
        .collect();

    logs.sort_by(|a, b| a.ts.cmp(&b.ts));

    let mut sleeps_by_guard: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    let mut guard_id = 0;
    let mut sleep_start = 0;

    for log in logs {
        match log.event {
            LogEvent::Start(id) => guard_id = id,
            LogEvent::Sleep => sleep_start = log.ts.min,
            LogEvent::Wake => {
                let mut sleeps = match sleeps_by_guard.get(&guard_id) {
                    Some(sleeps) => sleeps.clone(),
                    None => Vec::new(),
                };
                sleeps.push((sleep_start, log.ts.min));
                sleeps_by_guard.insert(guard_id, sleeps);
            }
        }
    }

    let mut max = 0;
    let mut answer = 0;
    for (guard_id, sleeps) in sleeps_by_guard {
        let all_mins = sleeps.iter().map(|(start, end)| *start..*end).flatten();

        match mode(all_mins) {
            Some((min, f)) => {
                if f > max {
                    max = f;
                    answer = min * guard_id
                }
            }
            None => (),
        }
    }

    println!("{}", answer);
    Ok(())
}
