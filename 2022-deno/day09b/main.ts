import { readInput } from "../common.ts";

type Vec = [number, number];

enum Dir {
  Up,
  Down,
  Left,
  Right,
}

function newVec(x = 0, y = 0): Vec {
  return [x, y];
}

function parseDir(s: string): Dir {
  switch (s) {
    case "U":
      return Dir.Up;
    case "D":
      return Dir.Down;
    case "L":
      return Dir.Left;
    case "R":
      return Dir.Right;
  }

  throw new Error(`invalid dir string: ${s}`);
}

function parseMotion(s: string): [Dir, number] {
  const toks = s.split(" ");
  return [parseDir(toks[0]), parseInt(toks[1])];
}

function posKey([x, y]: Vec): string {
  return `${x}.${y}`;
}

function add(a: Vec, b: Vec, out: Vec) {
  out[0] = a[0] + b[0];
  out[1] = a[1] + b[1];
}

function diff(to: Vec, from: Vec): Vec {
  return [to[0] - from[0], to[1] - from[1]];
}

function sign(n: number): number {
  return n < 0 ? -1 : 1;
}

function tail(vecs: Vec[]): Vec {
  return vecs[vecs.length - 1];
}

const motions = readInput().split("\n").map(parseMotion);
const knots: Vec[] = [];
for (let i = 0; i < 10; i++) {
  knots.push(newVec());
}

const visited = new Set<string>();
visited.add(posKey(tail(knots)));

for (const [dir, steps] of motions) {
  const delta: [number, number] = newVec();
  switch (dir) {
    case Dir.Up:
      delta[1] = -1;
      break;
    case Dir.Down:
      delta[1] = 1;
      break;
    case Dir.Left:
      delta[0] = -1;
      break;
    case Dir.Right:
      delta[0] = 1;
      break;
  }

  for (let i = 0; i < steps; i++) {
    add(knots[0], delta, knots[0]);

    for (let j = 1; j < knots.length; j++) {
      const toPrev = diff(knots[j - 1], knots[j]);

      if (toPrev[0] != 0 && toPrev[1] != 0) {
        if (Math.abs(toPrev[0]) + Math.abs(toPrev[1]) > 2) {
          add(knots[j], newVec(sign(toPrev[0]), sign(toPrev[1])), knots[j]);
        }
      } else if (Math.abs(toPrev[0]) > 1) {
        knots[j][0] += sign(toPrev[0]);
      } else if (Math.abs(toPrev[1]) > 1) {
        knots[j][1] += sign(toPrev[1]);
      }
    }

    visited.add(posKey(tail(knots)));
  }
}

console.log(visited.size);
