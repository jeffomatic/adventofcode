import { readInputLines } from "../common.ts";

const input = readInputLines(import.meta.url).map((s) => {
  const [len, w, h] = s.split("x").map((s) => parseInt(s));
  return [len, w, h];
});

const res = input.reduce((accum, dim) => {
  const [a, b, c] = dim.sort((a, b) => a - b);
  return accum + 2 * (a + b) + (a * b * c);
}, 0);

console.log(res);
