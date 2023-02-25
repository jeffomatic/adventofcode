import { readInput } from "../common.ts";

const lines = readInput().split("\n");
const rows: number[][] = [];
for (const line of lines) {
  const row: number[] = [];
  for (const h of line) {
    row.push(parseInt(h));
  }
  rows.push(row);
}

const height = lines.length;
const width = lines[0].length;
let best = -1;

for (let i = 0; i < height; i++) {
  for (let j = 0; j < width; j++) {
    let left = j;
    for (let n = j - 1; n >= 0; n--) {
      if (rows[i][n] >= rows[i][j]) {
        left = j - n;
        break;
      }
    }

    let right = width - j - 1;
    for (let n = j + 1; n < width; n++) {
      if (rows[i][n] >= rows[i][j]) {
        right = n - j;
        break;
      }
    }

    let top = i;
    for (let n = i - 1; n >= 0; n--) {
      if (rows[n][j] >= rows[i][j]) {
        top = i - n;
        break;
      }
    }

    let bottom = height - i - 1;
    for (let n = i + 1; n < height; n++) {
      if (rows[n][j] >= rows[i][j]) {
        bottom = n - i;
        break;
      }
    }

    best = Math.max(best, left * right * top * bottom);
  }
}

console.log(best);
