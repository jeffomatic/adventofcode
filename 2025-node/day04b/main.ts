import { example, input, readlines } from '../lib/util';

function updateGrid(original: boolean[][], toRemove: [number, number][]): boolean[][] {
  const res = structuredClone(original);
  for (const [x, y] of toRemove) {
    res[y][x] = false;
  }
  return res;
}

function iteration(grid: boolean[][]): [number, number][] {
  const res: [number, number][] = [];

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
        res.push([j, i]);
      }
    }
  }

  return res;
}

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
let grid: Array<Array<boolean>> = lines.map((line) => line.split('').map((c) => c == '@'));

let res = 0;
while (true) {
  const removeList = iteration(grid);
  if (removeList.length == 0) {
    break;
  }

  res += removeList.length;
  grid = updateGrid(grid, removeList);
}

console.log(res);
