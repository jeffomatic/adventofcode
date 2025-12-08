import { example, input } from '../lib/util';

// const lines = example({ noTrim: true }); // answer: 3263827
const lines = input({ noTrim: true });

const split = lines.map((l) => l.split(''));
const operators = split[split.length - 1];
let operatorIndex = 0;
let res = 0;

while (operatorIndex >= 0) {
  const nextOperatorIndex = operators.findIndex((c, i) => {
    return i > operatorIndex && (c == '+' || c == '*');
  });
  const maxDigits =
    nextOperatorIndex < 0
      ? operators.length - operatorIndex
      : nextOperatorIndex - operatorIndex - 1;

  const operands = [];
  for (let i = 0; i < maxDigits; i++) {
    const col = i + operatorIndex;
    let accum = '';
    for (let j = 0; j < split.length - 1; j++) {
      accum += split[j][col];
    }
    operands.push(parseInt(accum));
  }

  if (operators[operatorIndex] == '+') {
    res += operands.reduce((memo, n) => memo + n);
  } else {
    res += operands.reduce((memo, n) => memo * n);
  }

  operatorIndex = nextOperatorIndex;
}

console.log(res);
