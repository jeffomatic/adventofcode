import { intersection, readInput } from "../common.ts";

function pri(s: string): number {
  const v = s.charCodeAt(0);

  const a = "a".charCodeAt(0);
  if (v >= a) {
    return 1 + v - a;
  }

  const A = "A".charCodeAt(0);
  return 27 + v - A;
}

const lines = readInput().split("\n");
let res = 0;
for (let i = 0; i < lines.length - 2; i += 3) {
  const a = lines[i].split("");
  const b = lines[i + 1].split("");
  const c = lines[i + 2].split("");
  const common = intersection(new Set(a), intersection(new Set(b), new Set(c)));
  res += pri(Array.from(common)[0]);
}

console.log(res);
