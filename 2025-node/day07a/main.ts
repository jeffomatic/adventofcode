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

let beams = new Set([lines[0].indexOf('S')]);

let splits = 0;
for (const split of splitters) {
  let nextBeams = new Set<number>();
  for (const beam of beams) {
    if (split.includes(beam)) {
      splits++;
      nextBeams.add(beam - 1);
      nextBeams.add(beam + 1);
    } else {
      nextBeams.add(beam);
    }
  }
  beams = nextBeams;
}

console.log(splits);
