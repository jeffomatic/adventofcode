# aoc2021-rust

Here are my solutions to [Advent of Code 2020](https://adventofcode.com/2021). This repository is implemented in Rust.

## Running programs

Each exercise (two per day, except probably Day 25) is its own Cargo project. To run the `day01a` project, navigate to the top of this repository and type:

```
make day01a
```

Of course, you can substitute `day01a` with the name of a different directory. This works with any directory with a `day` prefix, so there is no need to edit the Makefile when adding new project directories.

The build rule for each directory will compile the binary and run it. You can just keep running the `make` rule for a nice edit-compile-run loop. If a file called `input` exists at the top of the project subdirectory, its contents will be piped into the running program. For most Advent of Code projects, there is a large, fixed string input, so having it available via stdin is helpful.

To run a *release build*, append `-release` to the make target, e.g.:

```
make day01-release
```

## Creating new folders

To start a new day's problem, I usually just copy the barebones template in `day0`:

```
cp -R day0 dayXYZ
```

Then I update `dayXYZ/Cargo.toml` so the package name is `dayXYZ` instead of `day0`.

Generally, the second project for each day closely resembles the first project, so I will copy the contents of the first project:

```
cp -R day01a day01b
```

As above, make sure to update `Cargo.toml` with an appropriate package name.

## Some common patterns

### Reading stdin

Almost every project uses this function to read stdin until EOF, and pack it into a single `String`.


```rust
fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
```

### Parsing input

Sometimes, Advent of Code projects have somewhat idiosyncratic text input that requires a bit of parsing. I've found text parsing in Rust to be a little more difficult than other languages, especially because it lacks a `sscanf`-style format language. It does have some regular expression libraries that are useful after some initial finagling. I use the `regex` crate, with some assistance from the `lazy_static` crate, which lets you avoid re-creating the regex evaluator object for every string you need to parse.

Here's a typical pattern that allows you to use `::parse<T>()` to convert a string into an object. (Here, `Segment` is just an example struct.)

```rust
use lazy_static::lazy_static;
use regex::Regex;

struct Segment {
    dir: char,
    length: i64,
}

impl FromStr for Segment {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<dir>[UDLR])(?P<length>\d+)").unwrap();
        }

        let caps = match RE.captures(s) {
            None => return Err(From::from("invalid input")),
            Some(caps) => caps,
        };

        return Ok(Segment {
            dir: caps["dir"].parse::<char>().unwrap(),
            length: caps["length"].parse::<i64>().unwrap(),
        });
    }
}

fn main() {
    let input = get_input();
    let lines: Vec<Vec<Segment>> = input
        .lines()
        .map(|line| line.split(",").map(|s| s.parse().unwrap()).collect())
        .collect();
    ...
}
```

## Debugging

### Launch

```
$ cd dayXYZ
$ lldb target/debug/dayXYZ
(lldb) b <line no>
(lldb) process launch -i ./input
...
```
