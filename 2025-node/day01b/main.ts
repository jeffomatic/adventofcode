import { readlines } from '../lib/util';

const lines = readlines(__dirname + '/input');
const instructions = lines.map((s) => {
  const n = parseInt(s.substring(1));

  if (s.startsWith('L')) {
    return n * -1;
  }

  return n;
});

let dial = 50;
let zeroCount = 0;
for (let n of instructions) {
  zeroCount += Math.floor(Math.abs(n) / 100);

  n = n % 100;
  if (n == 0) {
    continue;
  }

  const prev = dial;
  dial = (dial + n) % 100;
  if (dial < 0) {
    dial += 100;
  }

  if (dial == 0) {
    zeroCount += 1;
  } else if (prev != 0 && n > 0 && dial < prev) {
    zeroCount += 1;
  } else if (prev != 0 && n < 0 && dial > prev) {
    zeroCount += 1;
  }
}

console.log(zeroCount);
