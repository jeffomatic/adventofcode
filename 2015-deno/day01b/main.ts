import { readInput } from "../common.ts";

const input = readInput(import.meta.url);

let pos = 0;
for (let i = 0; i < input.length; i++) {
  pos += input[i] == "(" ? 1 : -1;
  if (pos == -1) {
    console.log(i + 1);
    break;
  }
}
