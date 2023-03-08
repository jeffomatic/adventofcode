import { readInput } from "../common.ts";
import * as vec2 from "../vec2.ts";
type Vec2 = vec2.Vec2;

const input = readInput(import.meta.url);

let a = vec2.make(0, 0);
let b = vec2.make(0, 0);
const visited = new Set<string>();
visited.add(vec2.key(a));

function parseDir(c: string): Vec2 {
  switch (c) {
    case "<":
      return vec2.make(-1, 0);

    case ">":
      return vec2.make(+1, 0);

    case "^":
      return vec2.make(0, -1);

    case "v":
      return vec2.make(0, +1);
  }

  throw new Error(`cannot parse string: ${c}`);
}

for (let i = 0; i < input.length - 1; i += 2) {
  a = vec2.add(a, parseDir(input[i]), a);
  b = vec2.add(b, parseDir(input[i + 1]), b);
  visited.add(vec2.key(a));
  visited.add(vec2.key(b));
}

console.log(visited.size);
