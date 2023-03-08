import { readInputLines } from "../common.ts";

const input = readInputLines(import.meta.url).map((s) => {
  const [len, w, h] = s.split("x").map((s) => parseInt(s));
  return [len, w, h];
});

const res = input.reduce((accum, [len, w, h]) => {
  const a = len * w;
  const b = len * h;
  const c = w * h;
  return accum + 2 * a + 2 * b + 2 * c + Math.min(a, b, c);
}, 0);

console.log(res);
