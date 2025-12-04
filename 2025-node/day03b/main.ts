import { input, example } from '../lib/util';

/*
987654321111111
811111111111119
234234234234278
818181911112111

Now, the joltages are much larger:

    In 987654321111111, the largest joltage can be found by turning on everything except some 1s at the end to produce 987654321111.
    In the digit sequence 811111111111119, the largest joltage can be found by turning on everything except some 1s, producing 811111111119.
    In 234234234234278, the largest joltage can be found by turning on everything except a 2 battery, a 3 battery, and another 2 battery near the start to produce 434234234278.
    In 818181911112111, the joltage 888911112111 is produced by turning on everything except some 1s near the front.

The total output joltage is now much larger: 987654321111 + 811111111119 + 434234234278 + 888911112111 = 3121910778619.
*/

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

// const lines = example();
const lines = input();
const banks = lines.map((line) => line.split('').map((c) => parseInt(c)));

let res = 0;
const maxDigits = 12;
for (const bank of banks) {
  let remaining = bank;
  let subtotal = 0;
  for (let digit = 0; digit < maxDigits; digit++) {
    const searchSpace = remaining.length - (maxDigits - digit - 1);
    const [found, foundIndex] = maxOf(remaining.slice(0, searchSpace));
    subtotal += Math.pow(10, maxDigits - digit - 1) * found;
    remaining = remaining.slice(foundIndex + 1);
  }

  res += subtotal;
}

console.log(res);
