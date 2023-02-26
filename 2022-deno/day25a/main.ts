import { readInputLines } from "../common.ts";

function decode(s: string): number {
  let val = 0;

  for (let i = 0; i < s.length; i++) {
    const base = Math.pow(5, i);
    const c = s[s.length - 1 - i];
    switch (c) {
      case "=":
        val -= base * 2;
        break;

      case "-":
        val -= base;
        break;

      default:
        val += base * parseInt(c);
        break;
    }
  }

  return val;
}

function encode(n: number): string {
  // First, convert to base 5
  const digits: number[] = [];
  while (n > 0) {
    const rem = n % 5;
    digits.push(rem);
    n = Math.floor(n / 5);
  }

  // Add a leading zero to make array traversal a little easier
  digits.push(0);

  // Next, use a reverse-borrow system to push 4s and 5s to the next digit
  for (let i = digits.length - 1; i >= 0; i--) {
    const d1 = digits[i];
    if (d1 < 3) {
      continue;
    }

    digits[i] -= 5;

    for (let j = i + 1; j < digits.length; j++) {
      digits[j] += 1;
      if (digits[j] < 3) {
        break;
      }

      digits[j] = -2;
    }
  }

  // Trim the leading zero if necessary
  if (digits[digits.length - 1] == 0) {
    digits.pop();
  }

  return digits.reverse().map((d) =>
    d == -2 ? "=" : d == -1 ? "-" : d.toString()
  ).join("");
}

const testCases: [string, number][] = [
  ["1", 1],
  ["2", 2],
  ["1=", 3],
  ["1-", 4],
  ["10", 5],
  ["11", 6],
  ["12", 7],
  ["2=", 8],
  ["2-", 9],
  ["20", 10],
  ["1=0", 15],
  ["1-0", 20],
  ["1=11-2", 2022],
  ["1-0---0", 12345],
  ["1121-1110-1=0", 314159265],
  ["1=-0-2", 1747],
  ["12111", 906],
  ["2=0=", 198],
  ["21", 11],
  ["2=01", 201],
  ["111", 31],
  ["20012", 1257],
  ["112", 32],
  ["1=-1=", 353],
  ["1-12", 107],
  ["12", 7],
  ["1=", 3],
  ["122", 37],
];

for (const [s, n] of testCases) {
  if (decode(s) != n) {
    console.log(`decode(${s}) != ${n}`);
  }

  if (encode(n) != s) {
    console.log(`encode(${n}) != ${s} (got ${encode(n)})`);
  }
}

const res = encode(
  readInputLines().map((s) => decode(s)).reduce((sum, n) => sum + n),
);
console.log(res);
