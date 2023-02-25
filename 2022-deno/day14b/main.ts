import { readInputLines } from "../common.ts";
import * as vec2 from "../vec2.ts";
type Vec2 = vec2.Vec2;

const solid = new Set<string>();
let minX = Number.POSITIVE_INFINITY;
let maxX = 0;
let maxY = 0;

for (const line of readInputLines()) {
  const points = line
    .split(" -> ")
    .map((s) => s.split(",").map((num) => parseInt(num)) as Vec2);
  for (let i = 0; i < points.length - 1; i++) {
    const src = points[i];
    const dst = points[i + 1];

    for (const p of vec2.march(src, dst)) {
      solid.add(vec2.key(p));
      minX = Math.min(minX, p[0]);
      maxX = Math.max(maxX, p[0]);
      maxY = Math.max(maxY, p[1]);
    }
  }
}

const start = vec2.make(500, 0);
const down = vec2.make(0, 1);
const downLeft = vec2.make(-1, 1);
const downRight = vec2.make(1, 1);
let rested = 0;

outer:
while (true) {
  let cur = vec2.copy(start);
  while (true) {
    // If we exceed the boundaries of existing solid surfaces, then the sand
    // will flow into the abyss, which means we're done.
    if (cur[1] == maxY + 1) {
      solid.add(vec2.key(cur));
      rested += 1;
      break;
    }

    let next = vec2.add(cur, down);
    if (!solid.has(vec2.key(next))) {
      cur = next;
      continue;
    }

    next = vec2.add(cur, downLeft);
    if (!solid.has(vec2.key(next))) {
      cur = next;
      continue;
    }

    next = vec2.add(cur, downRight);
    if (!solid.has(vec2.key(next))) {
      cur = next;
      continue;
    }

    solid.add(vec2.key(cur));
    rested += 1;

    if (vec2.equal(cur, start)) {
      break outer;
    }

    break;
  }
}

console.log(rested);
