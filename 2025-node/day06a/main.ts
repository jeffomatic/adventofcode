import { example, input } from '../lib/util';

// const lines = example(); // answer: 4277556
const lines = input();
const split = lines.map((s) => s.trim().split(/\s+/));

let res = 0;

const formulaCount = split[0].length;
const operandCount = split.length - 1;

for (let i = 0; i < formulaCount; i++) {
  const operands: number[] = [];
  for (let j = 0; j < operandCount; j++) {
    operands.push(parseInt(split[j][i]));
  }

  const op = split[split.length - 1][i];
  switch (op) {
    case '+':
      res += operands.reduce((memo, n) => memo + n);
      break;
    case '*':
      res += operands.reduce((memo, n) => memo * n);
      break;
    default:
      throw new Error(`invalid operatior ${op}`);
  }
}

console.log(res);
