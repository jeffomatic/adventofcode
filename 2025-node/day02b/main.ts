import { example, input } from '../lib/util';

function isInvalid(n: number): boolean {
  const s = String(n);

  for (let i = 1; i <= Math.floor(s.length / 2); i++) {
    if (s.length % i != 0) {
      continue;
    }

    const pattern = s.substring(0, i);
    const testString = pattern.repeat(s.length / i);

    if (testString == s) {
      return true;
    }
  }

  return false;
}

// const lines = example()
const lines = input();

const ranges: Array<[number, number]> = lines[0]
  .split(',')
  .map((s) => s.split('-').map((n) => parseInt(n)) as [number, number]);

const doubles: Array<number> = [];

for (const [a, b] of ranges) {
  for (let i = a; i <= b; i++) {
    if (isInvalid(i)) {
      doubles.push(i);
    }
  }
}

console.log(doubles.reduce((memo, n) => memo + n, 0));
