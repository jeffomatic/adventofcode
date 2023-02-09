import { readInput } from "../common.ts";

function overlapping(
  [a, b, c, d]: [number, number, number, number],
): boolean {
  if (a <= c && c <= b) {
    return true;
  }

  if (c <= a && a <= d) {
    return true;
  }

  return false;
}

function parseLine(s: string): [number, number, number, number] {
  const [x, y] = s.split(",");
  const [a, b] = x.split("-").map((s) => parseInt(s));
  const [c, d] = y.split("-").map((s) => parseInt(s));
  return [a, b, c, d];
}

const res = readInput()
  .split("\n")
  .map(parseLine)
  .filter(overlapping)
  .length;
console.log(res);
