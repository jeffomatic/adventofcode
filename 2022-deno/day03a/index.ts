import { readInput } from "../common.ts";

function pri(s: string): number {
  const v = s.charCodeAt(0);

  const a = "a".charCodeAt(0);
  if (v >= a) {
    return 1 + v - a;
  }

  const A = "A".charCodeAt(0);
  return 27 + v - A;
}

let res = 0;
for (const line of readInput().split("\n")) {
  let i = 0;
  const items = new Set<string>();
  for (; i < line.length / 2; i++) {
    items.add(line[i]);
  }
  for (; i < line.length; i++) {
    if (items.has(line[i])) {
      items.delete(line[i]);
      res += pri(line[i]);
    }
  }
}

console.log(res);
