# aoc2025-node

Here are my solutions to [Advent of Code 2025](https://adventofcode.com/2025). This repository is implemented in TypeScript, using the NodeJS runtime.

## Running programs

Each exercise (two per day, except probably Day 12) contains a file called main.ts that contains the solution. To run the `day01a` project, navigate to the top of this repository and type:

```
make day01a
```

Of course, you can substitute `day01a` with the name of a different directory. This works with any directory with a `day` prefix, so there is no need to edit the Makefile when adding new project directories.

The build rule for each directory will compile the binary and run it. You can just keep running the `make` rule for a nice edit-compile-run loop.

## Creating new folders

To start a new day's problem, I usually just copy `template`:

```
cp -R template dayXYZ
```

## Debugging

You can perform breakpoint debugging from VSCode by opening the "Run and Debug" pane, and then run the "Debug Current TypeScript File" task.