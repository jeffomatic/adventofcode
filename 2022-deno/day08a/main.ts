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
const visible = new Set<number>();

function check(i: number, j: number, higherThan: number): number {
  if (rows[i][j] <= higherThan) {
    return higherThan;
  }

  visible.add(i * width + j);
  return rows[i][j];
}

// left
for (let i = 0; i < height; i++) {
  let highest = -1;
  for (let j = 0; j < width; j++) {
    highest = check(i, j, highest);
    if (highest == 9) {
      break;
    }
  }
}

// right
for (let i = 0; i < height; i++) {
  let highest = -1;
  for (let j = width - 1; j >= 0; j--) {
    highest = check(i, j, highest);
    if (highest == 9) {
      break;
    }
  }
}

// top
for (let j = 0; j < width; j++) {
  let highest = -1;
  for (let i = 0; i < height; i++) {
    highest = check(i, j, highest);
    if (highest == 9) {
      break;
    }
  }
}

// bottom
for (let j = 0; j < width; j++) {
  let highest = -1;
  for (let i = height - 1; i >= 0; i--) {
    highest = check(i, j, highest);
    if (highest == 9) {
      break;
    }
  }
}

console.log(visible.size);
