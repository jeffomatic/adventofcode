# aoc2018

Solution to [Advent of Code 2018](http://adventofcode.com/2018).

## How to run programs

Each program has its own directory. To run the program, navigate to the root of the repo and type:

```
make <directory name>
```

This works with any directory that contains a file called `main.ml`. If the directory also contains a file called `input`, the contents of that file will be piped into the program's stdin.

## Debugging

To run a program in the OCaml debugger:

```
make debug-<directory name>
```

When debugging, it helps to create a file called [`.ocamldebug`](https://caml.inria.fr/pub/docs/manual-ocaml/debugger.html#sec379) in the program directory. This file contains initialization commands that will be run when the debugging session starts.

For example, the following commands setup the program's stdin and set a breakpoint:

```
set arguments < input
break @ Main 80
```
