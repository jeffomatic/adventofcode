import { readInput } from "../common.ts";

const input = readInput().split("");
let uniqStart = 0;
const msgLen = 14;

for (let i = 0; i < input.length; i++) {
  const cur = input[i];
  for (let j = i - (msgLen - 1); j < i; j++) {
    if (cur == input[j]) {
      uniqStart = Math.max(uniqStart, j + 1);
    }
  }

  if (uniqStart == i - (msgLen - 1)) {
    console.log(i + 1);
    break;
  }
}
