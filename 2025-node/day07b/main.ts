import { example, input } from '../lib/util';

// const lines = example();
const lines = input();

const splitters: number[][] = lines.map((line) => {
  const chars = line.split('');
  return chars.reduce((memo, c, index) => {
    if (c == '^') {
      memo.push(index);
    }
    return memo;
  }, [] as number[]);
});

const cache = new Map<string, number>();

function countSplits(loc: number, depth: number): number {
  if (depth >= splitters.length) {
    return 1;
  }

  const k = `${loc}-${depth}`;
  const cached = cache.get(k);
  if (cached) {
    return cached;
  }

  let res = undefined;
  if (splitters[depth].includes(loc)) {
    res = countSplits(loc - 1, depth + 1) + countSplits(loc + 1, depth + 1);
  } else {
    res = countSplits(loc, depth + 1);
  }

  cache.set(k, res);
  return res;
}

const loc = lines[0].indexOf('S');
const res = countSplits(loc, 1);
console.log(res);
