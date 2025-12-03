import { readlines } from '../lib/util';

const lines = readlines(__dirname + '/input');
if (lines[0] === undefined) {
  throw new Error('invalid input');
}

const ranges: Array<[number, number]> = lines[0]
  .split(',')
  .map((s) => s.split('-').map((n) => parseInt(n)) as [number, number]);

const doubles: Array<number> = [];

for (const [a, b] of ranges) {
  for (let i = a; i <= b; i++) {
    const s = String(i);
    if (s.length % 2 != 0) {
      continue;
    }

    if (s.substring(0, s.length / 2) == s.substring(s.length / 2)) {
      doubles.push(i);
    }
  }
}

console.log(doubles.reduce((memo, n) => memo + n, 0));
