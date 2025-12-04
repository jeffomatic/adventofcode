import { input, example } from '../lib/util';

function maxOf(nums: number[]): [number, number] {
  let max = -Infinity;
  let maxIndex = -1;

  for (let i = 0; i < nums.length; i++) {
    const n = nums[i];
    if (n > max) {
      max = n;
      maxIndex = i;
    }
  }

  return [max, maxIndex];
}

// const lines = example()
const lines = input();
const banks = lines.map((line) => line.split('').map((c) => parseInt(c)));

let res = 0;
for (const bank of banks) {
  const [first, firstIndex] = maxOf(bank.slice(0, bank.length - 1));
  const [last, _] = maxOf(bank.slice(firstIndex + 1));
  res += 10 * first + last;
}

console.log(res);
