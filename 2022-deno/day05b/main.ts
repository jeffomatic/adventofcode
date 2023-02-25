import { readInput } from "../common.ts";

type Instruction = [number, number, number]; // num, src, dst

function parseInstructions(s: string): Instruction[] {
  return s.split("\n").map((line) => {
    const toks = line.split(" ");
    return [parseInt(toks[1]), parseInt(toks[3]) - 1, parseInt(toks[5]) - 1];
  });
}

function parseStacks(s: string): string[][] {
  const rawLines = s.split("\n");
  const lines = rawLines.slice(0, rawLines.length - 1);
  const cols = Math.ceil(lines[lines.length - 1].length / 4);

  const res: string[][] = [];
  for (let i = 0; i < cols; i++) {
    res.push([]);
  }

  for (const line of lines) {
    for (let i = 0; i < cols; i++) {
      const c = line[i * 4 + 1];
      if (c == " ") {
        continue;
      }
      res[i].push(c);
    }
  }

  return res;
}

const [stackSrc, insSrc] = readInput().split("\n\n");
const stacks = parseStacks(stackSrc);
const instructions = parseInstructions(insSrc);

for (const [num, src, dst] of instructions) {
  const srcArray = stacks[src];
  stacks[src] = srcArray.slice(num);
  stacks[dst] = srcArray.slice(0, num).concat(stacks[dst]);
}

const res = stacks.map((s) => s[0]).join("");
console.log(res);
