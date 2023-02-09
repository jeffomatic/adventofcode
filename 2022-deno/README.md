# Advent of Code 2022 (TypeScript/Deno)

Here are my solutions to [Advent of Code 2022](https://adventofcode.com/2022).
This repository is implemented in TypeScript, using the
[Deno runtime](https://deno.land).

## Running programs

Each exercise (two per day, except Day 25) has its own directory. To run the
`day01a` project, navigate to the directory containing this README and type:

```
make day01a
```

Of course, you can substitute `day01a` with the name of a different directory.
This works with any directory with a `day` prefix, so there is no need to edit
the Makefile when adding new project directories.

The `make` rule for each directory will run a script in that directory called
`index.ts`. You can just keep running this rule for a nice edit-run loop. If a
file called `input` exists within the directory, its contents will be piped into
the running program. For most Advent of Code projects, there is a large, fixed
string input, so having it available via stdin is helpful.

## Creating new folders

To start a new day's problem, I just add a new folder with the appropriate name.

```
mkdir dayXYZ
```

Generally, the second project for each day closely resembles the first project,
so I will copy the contents of the first project:

```
cp -R day01a day01b
```

## Utilities

See `common.ts` for a list of commonly-used functions. Some examples:

- `readInput()`: Dump the contents of stdin into a string.
- `printFull()`: JSON-serializes the provided argument, and then prints the JSON
  to stdout. Use in place of `console.log` for large data structures.

## Debugging

TBD...haven't needed it beyond `console.log` yet, but I'm sure I will.
