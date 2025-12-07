import { example, input } from '../lib/util';

// const lines = example();
const lines = input();

const blank = lines.indexOf('');
const ranges: [number, number][] = lines
  .slice(0, blank)
  .map((s) => s.split('-').map((c) => parseInt(c)) as [number, number]);
const ingredients = lines.slice(blank + 1).map((s) => parseInt(s));

function inAnyRange(n: number, ranges: [number, number][]): boolean {
  for (const [min, max] of ranges) {
    if (min <= n && n <= max) {
      return true;
    }
  }

  return false;
}

let count = 0;
for (const ing of ingredients) {
  if (inAnyRange(ing, ranges)) {
    count++;
  }
}

console.log(count);
