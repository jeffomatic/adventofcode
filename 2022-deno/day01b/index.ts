import { printFull, readInput } from "../common.ts";

const res = readInput()
  .split("\n\n")
  .map((chunk) =>
    chunk.split("\n").map((s) => parseInt(s)).reduce((memo, v) => memo + v, 0)
  )
  .sort((a, b) => a - b)
  .reverse()
  .slice(0, 3)
  .reduce((memo, v) => memo + v);
printFull(res);
