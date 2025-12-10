import { example, input } from '../lib/util';

// const lines = example(); // 50
const lines = input();
const points: [number, number][] = lines.map(
  (line) => line.split(',').map((n) => parseInt(n)) as [number, number],
);

let best = -1;
for (let i = 0; i < points.length - 1; i++) {
  for (let j = i; j < points.length; j++) {
    const [ax, ay] = points[i];
    const [bx, by] = points[j];
    const w = Math.max(ax, bx) - Math.min(ax, bx) + 1;
    const h = Math.max(ay, by) - Math.min(ay, by) + 1;
    best = Math.max(best, w * h);
  }
}

console.log(best);
