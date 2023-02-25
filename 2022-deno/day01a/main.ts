import { printFull, readInput } from "../common.ts";

const res = readInput()
  .split("\n\n")
  .map((chunk) =>
    chunk.split("\n").map((s) => parseInt(s)).reduce((memo, v) => memo + v, 0)
  )
  .sort((a, b) => a - b)
  .reverse()[0];
printFull(res);
