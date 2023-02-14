import { readInputLines } from "../common.ts";
import * as vec3 from "../vec3.ts";
type Vec3 = vec3.Vec3;

const items: Record<string, Vec3> = {};
for (const line of readInputLines()) {
  const [x, y, z] = line.split(",").map((s) => parseInt(s));
  const pos: Vec3 = [x, y, z];
  items[vec3.key(pos)] = pos;
}

const dirs: Vec3[] = [
  [-1, 0, 0],
  [1, 0, 0],
  [0, -1, 0],
  [0, 1, 0],
  [0, 0, -1],
  [0, 0, 1],
];

let res = 0;
for (const p of Object.values(items)) {
  for (const dir of dirs) {
    const neighbor = vec3.add(p, dir);
    if (items[vec3.key(neighbor)] == undefined) {
      res += 1;
    }
  }
}

console.log(res);
