import { example, input, readlines } from '../lib/util';

const neighbors = [
  [-1, -1],
  [-1, 0],
  [-1, 1],
  [0, -1],
  [0, 1],
  [1, -1],
  [1, 0],
  [1, 1],
];
// const lines = example();
const lines = input();
const grid: Array<Array<boolean>> = lines.map((line) => line.split('').map((c) => c == '@'));

let res = 0;

for (let i = 0; i < grid.length; i++) {
  const row = grid[i];
  for (let j = 0; j < row.length; j++) {
    if (!grid[i][j]) {
      continue;
    }

    let count = 0;
    for (const [dx, dy] of neighbors) {
      const x = j + dx;
      const y = i + dy;
      if (x < 0 || y < 0 || row.length <= x || grid.length <= y) {
        continue;
      }

      if (grid[y][x]) {
        count++;
      }
    }

    if (count < 4) {
      res++;
    }
  }
}

console.log(res);
