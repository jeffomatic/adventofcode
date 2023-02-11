import { readInput } from "../common.ts";

enum Dir {
  Up,
  Down,
  Left,
  Right,
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

function posKey([x, y]: [number, number]): string {
  return `${x}.${y}`;
}

function diff(to: [number, number], from: [number, number]): [number, number] {
  return [to[0] - from[0], to[1] - from[1]];
}

function sign(n: number): number {
  return n < 0 ? -1 : 1;
}

const motions = readInput().split("\n").map(parseMotion);
const head: [number, number] = [0, 0];
const tail: [number, number] = [0, 0];
const visited = new Set<string>();
visited.add(posKey(tail));

for (const [dir, steps] of motions) {
  const delta: [number, number] = [0, 0];
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
    head[0] += delta[0];
    head[1] += delta[1];

    const toHead = diff(head, tail);

    if (toHead[0] != 0 && toHead[1] != 0) {
      if (Math.abs(toHead[0]) + Math.abs(toHead[1]) > 2) {
        tail[0] += sign(toHead[0]);
        tail[1] += sign(toHead[1]);
      }
    } else if (Math.abs(toHead[0]) > 1) {
      tail[0] += sign(toHead[0]);
    } else if (Math.abs(toHead[1]) > 1) {
      tail[1] += sign(toHead[1]);
    }

    visited.add(posKey(tail));
  }
}

console.log(visited.size);
