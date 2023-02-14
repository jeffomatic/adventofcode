import { readInputLines } from "../common.ts";
import * as vec3 from "../vec3.ts";
type Vec3 = vec3.Vec3;

const items: Record<string, Vec3> = {};
let minX = Number.POSITIVE_INFINITY;
let minY = Number.POSITIVE_INFINITY;
let minZ = Number.POSITIVE_INFINITY;
let maxX = Number.NEGATIVE_INFINITY;
let maxY = Number.NEGATIVE_INFINITY;
let maxZ = Number.NEGATIVE_INFINITY;

for (const line of readInputLines()) {
  const [x, y, z] = line.split(",").map((s) => parseInt(s));
  const pos: Vec3 = [x, y, z];
  items[vec3.key(pos)] = pos;
  minX = Math.min(minX, x);
  minY = Math.min(minY, y);
  minZ = Math.min(minZ, z);
  maxX = Math.max(maxX, x);
  maxY = Math.max(maxY, y);
  maxZ = Math.max(maxZ, z);
}

const dirs: Vec3[] = [
  [-1, 0, 0],
  [1, 0, 0],
  [0, -1, 0],
  [0, 1, 0],
  [0, 0, -1],
  [0, 0, 1],
];

const visited = new Set<string>();

// prevent search from escaping a shell 2 units outside of the bounds
for (let x = minX - 2; x <= maxX + 2; x++) {
  for (let y = minY - 2; y <= maxY + 2; y++) {
    for (let z = minZ - 2; z <= maxZ + 2; z++) {
      if (
        x == minX - 2 || y == minY - 2 || z == minZ - 2 ||
        x == maxX + 2 || y == maxY + 2 || z == maxZ + 2
      ) {
        visited.add(vec3.key(vec3.make(x, y, z)));
      }
    }
  }
}

// Seed the search in the shell immediately outisde of the items
const q: Vec3[] = [];
for (let x = minX - 1; x <= maxX + 1; x++) {
  for (let y = minY - 1; y <= maxY + 1; y++) {
    for (let z = minZ - 1; z <= maxZ + 1; z++) {
      if (
        x == minX - 1 || y == minY - 1 || z == minZ - 1 ||
        x == maxX + 1 || y == maxY + 1 || z == maxZ + 1
      ) {
        const pos = vec3.make(x, y, z);
        visited.add(vec3.key(pos));
        q.push(pos);
      }
    }
  }
}

let res = 0;
while (q.length > 0) {
  const pos = q.pop()!;
  for (const dir of dirs) {
    const other = vec3.add(pos, dir);
    const key = vec3.key(other);
    if (items[key] != undefined) {
      res += 1;
      continue;
    }

    if (!visited.has(key)) {
      q.push(other);
      visited.add(key);
    }
  }
}

console.log(res);
