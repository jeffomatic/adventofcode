import { readInput } from "../common.ts";

import * as vec2 from "../vec2.ts";

const input = readInput(import.meta.url);

const pos = vec2.make(0, 0);
const visited = new Set<string>();
visited.add(vec2.key(pos));
for (const c of input) {
  switch (c) {
    case "<":
      vec2.add(pos, vec2.make(-1, 0), pos);
      break;
    case ">":
      vec2.add(pos, vec2.make(+1, 0), pos);
      break;
    case "^":
      vec2.add(pos, vec2.make(0, -1), pos);
      break;
    case "v":
      vec2.add(pos, vec2.make(0, +1), pos);
      break;
  }
  visited.add(vec2.key(pos));
}

console.log(visited.size);
