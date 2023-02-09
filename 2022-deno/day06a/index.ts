import { readInput } from "../common.ts";

const input = readInput().split("");
let uniqStart = 0;

for (let i = 0; i < input.length; i++) {
  const cur = input[i];
  for (let j = i - 3; j < i; j++) {
    if (cur == input[j]) {
      uniqStart = Math.max(uniqStart, j + 1);
    }
  }

  if (uniqStart == i - 3) {
    console.log(i + 1);
    break;
  }
}
