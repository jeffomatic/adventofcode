use intcode;
use std::fs;
use std::io::{BufRead, Write};

fn flush() {
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
}

fn run_interactive(program: &Vec<i64>) {
    let mut cpu = intcode::Computer::new(&program);

    // boot program
    let res = cpu.run(&Vec::new());
    print!("{}> ", intcode::stream_to_string(&res.output));
    flush();

    // run interactively
    for s in std::io::stdin().lock().lines() {
        let s = s.unwrap() + "\n";
        let res = cpu.run(&intcode::stream_from_string(s.as_str()));
        if !res.unused_input.is_empty() {
            println!(
                "warning: unused input \"{}\"",
                intcode::stream_to_string(&res.unused_input)
            );
            flush();
        }

        print!("{}", intcode::stream_to_string(&res.output));

        match res.state {
            intcode::State::BlockedOnRead => {
                print!("> ");
                flush();
            }
            intcode::State::Halted => {
                print!("halted");
                break;
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }

    fn to_command(&self) -> String {
        match self {
            Dir::N => "north".to_string(),
            Dir::S => "south".to_string(),
            Dir::E => "east".to_string(),
            Dir::W => "west".to_string(),
        }
    }
}

// We only care about items that won't end the game
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Item {
    Mutex,
    Loom,
    Semiconductor,
    Ornament,
    Sand,
    Asterisk,
    Wreath,
    DarkMatter,
}

impl Item {
    fn to_name(&self) -> String {
        (match self {
            Item::Mutex => "mutex",
            Item::Loom => "loom",
            Item::Semiconductor => "semiconductor",
            Item::Ornament => "ornament",
            Item::Sand => "sand",
            Item::Asterisk => "asterisk",
            Item::Wreath => "wreath",
            Item::DarkMatter => "dark matter",
        })
        .to_string()
    }

    fn all() -> Vec<Item> {
        vec![
            Item::Mutex,
            Item::Loom,
            Item::Semiconductor,
            Item::Ornament,
            Item::Sand,
            Item::Asterisk,
            Item::Wreath,
            Item::DarkMatter,
        ]
    }

    fn combos() -> Vec<Vec<Item>> {
        let all = Self::all();
        let mut combos = Vec::new();

        for mask in 1..(1 << all.len()) {
            let mut v = Vec::new();
            for n in 0..all.len() {
                if (1 << n) & mask != 0 {
                    v.push(all[n]);
                }
            }
            combos.push(v);
        }

        combos
    }
}

fn item_to_path(item: Item) -> Vec<Dir> {
    match item {
        Item::Mutex => vec![Dir::N, Dir::N, Dir::N],
        Item::Loom => vec![Dir::N, Dir::E, Dir::N],
        Item::Semiconductor => vec![Dir::E],
        Item::Ornament => vec![Dir::E, Dir::E],
        Item::Sand => vec![Dir::W, Dir::W],
        Item::Asterisk => vec![Dir::W, Dir::W, Dir::S, Dir::E],
        Item::Wreath => vec![Dir::W, Dir::W, Dir::S, Dir::E, Dir::N],
        Item::DarkMatter => vec![Dir::W, Dir::W, Dir::N],
    }
}

fn reverse_path(path: &Vec<Dir>) -> Vec<Dir> {
    path.iter().rev().map(|d| d.opposite()).collect()
}

fn path_to_commands(path: &Vec<Dir>) -> Vec<String> {
    path.iter().map(|d| d.to_command() + "\n").collect()
}

fn fetch_item_commands(item: Item) -> Vec<String> {
    let mut path = path_to_commands(&item_to_path(item));
    path.push("take ".to_string() + &item.to_name() + "\n");
    path.append(&mut path_to_commands(&reverse_path(&item_to_path(item))));
    path
}

fn path_to_security_checkpoint() -> Vec<Dir> {
    vec![Dir::W, Dir::W, Dir::N, Dir::E, Dir::E]
}

fn try_combo(program: &Vec<i64>, combo: &Vec<Item>) -> String {
    let mut commands: Vec<String> = combo
        .iter()
        .map(|item| fetch_item_commands(*item))
        .flatten()
        .collect();
    commands.append(&mut path_to_commands(&path_to_security_checkpoint()));

    let mut cpu = intcode::Computer::new(program);
    let input = intcode::stream_from_string(&commands.concat());
    let res = cpu.run(&input);
    intcode::stream_to_string(&res.output)
}

fn main() {
    let program: Vec<i64> =
        intcode::program_from_string(&fs::read_to_string("./day25a/program").unwrap());
    // run_interactive(&program);

    for c in Item::combos().iter() {
        let s = try_combo(&program, &c);
        if !s.contains("Droids on this ship are lighter")
            && !s.contains("Droids on this ship are heavier")
        {
            println!("{}", s);
            println!("{:?}", c);
            break;
        }
    }
}
