import { readInput } from "../common.ts";

function fullyContained(
  [a, b, c, d]: [number, number, number, number],
): boolean {
  if (a <= c && d <= b) {
    return true;
  }

  if (c <= a && b <= d) {
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
  .filter(fullyContained)
  .length;
console.log(res);
