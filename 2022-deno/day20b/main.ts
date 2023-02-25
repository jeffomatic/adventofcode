import { readInputLines } from "../common.ts";

function seqToArray(
  start: number,
  vals: number[],
  seqNext: number[],
): number[] {
  const res: number[] = [vals[start]];
  for (let pos = seqNext[start]; pos != start; pos = seqNext[pos]) {
    res.push(vals[pos]);
  }

  return res;
}

const multiplier = 811589153;
const rawVals = readInputLines().map((s) => parseInt(s));
const vals = rawVals.map((v) => v * multiplier);
const len = vals.length;
const zeroIndex = vals.indexOf(0);

function mix(
  vals: number[],
  seqPrev: number[],
  seqNext: number[],
) {
  for (let i = 0; i < len; i++) {
    let steps = vals[i] % (len - 1);
    if (steps == 0) {
      continue;
    }

    // Remove the current item from the sequence
    const neighborPrev = seqPrev[i];
    const neighborNext = seqNext[i];
    seqNext[neighborPrev] = neighborNext;
    seqPrev[neighborNext] = neighborPrev;

    // Find a landing spot for the current item
    let insertAt = 0;
    if (steps > 0) {
      insertAt = neighborNext;
      for (let j = 0; j < steps; j++) {
        insertAt = seqNext[insertAt];
      }
    } else {
      steps = Math.abs(steps);
      insertAt = i;
      for (let j = 0; j < steps; j++) {
        insertAt = seqPrev[insertAt];
      }
    }

    const newPrev = seqPrev[insertAt];
    seqNext[newPrev] = i;
    seqPrev[i] = newPrev;
    seqNext[i] = insertAt;
    seqPrev[insertAt] = i;
  }
}

const seqPrev: number[] = [];
const seqNext: number[] = [];
for (let i = 0; i < len; i++) {
  seqPrev[i] = i > 0 ? i - 1 : len - 1;
  seqNext[i] = i < len - 1 ? i + 1 : 0;
}

for (let i = 0; i < 10; i++) {
  mix(vals, seqPrev, seqNext);
}

const final = seqToArray(zeroIndex, vals, seqNext);
let res = 0;
for (const n of [1000, 2000, 3000]) {
  const val = final[n % len];
  res += val;
}

console.log(res);
