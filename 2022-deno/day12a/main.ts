import { readInputLines } from "../common.ts";

type Vec = [number, number];

const map: number[][] = [];
let start: Vec = [0, 0];
let end: Vec = [0, 0];

const lines = readInputLines();
for (let i = 0; i < lines.length; i++) {
  const cols = lines[i].split("");
  map.push(new Array(cols.length));

  for (let j = 0; j < cols.length; j++) {
    let code = cols[j];

    if (cols[j] == "S") {
      code = "a";
      start = [i, j];
    } else if (cols[j] == "E") {
      code = "z";
      end = [i, j];
    }

    map[i][j] = code.charCodeAt(0) - "a".charCodeAt(0);
  }
}

function vecKey(v: Vec): string {
  return `${v[0]}.${v[1]}`;
}

function vecAdd(a: Vec, b: Vec): Vec {
  return [a[0] + b[0], a[1] + b[1]];
}

function vecEquals(a: Vec, b: Vec): boolean {
  return a[0] == b[0] && a[1] == b[1];
}

const q: Vec[] = [start];
const dist: Record<string, number> = {
  [vecKey(start)]: 0,
};
const dirs: Vec[] = [
  [-1, 0], // west
  [1, 0], // east
  [0, -1], // north
  [0, 1], // south
];
const height = map.length;
const width = map[0].length;

search:
while (q.length > 0) {
  const cur = q.shift()!;
  const curHeight = map[cur[0]][cur[1]];
  const curDist = dist[vecKey(cur)];

  for (const dir of dirs) {
    const next = vecAdd(cur, dir);

    // boundary check
    if (
      next[0] < 0 || height <= next[0] ||
      next[1] < 0 || width <= next[1]
    ) {
      continue;
    }

    // have we already added this to the queue?
    if (dist[vecKey(next)] != undefined) {
      continue;
    }

    const nextHeight = map[next[0]][next[1]];
    if (nextHeight - curHeight > 1) {
      continue;
    }

    if (vecEquals(next, end)) {
      console.log(curDist + 1);
      break search;
    }

    dist[vecKey(next)] = curDist + 1;
    q.push(next);
  }
}
