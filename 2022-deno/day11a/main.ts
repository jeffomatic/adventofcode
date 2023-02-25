import { readInput } from "../common.ts";

enum OpType {
  Add,
  Mul,
  Square,
}

type AddOp = {
  type: OpType.Add;
  val: number;
};

type MulOp = {
  type: OpType.Mul;
  val: number;
};

type SquareOp = {
  type: OpType.Square;
};

type Op = AddOp | MulOp | SquareOp;

function parseOp(s: string): Op {
  if (s.endsWith("old * old")) {
    return { type: OpType.Square };
  }

  const toks = s.split(" ");
  const op = toks[toks.length - 2];
  const val = parseInt(toks[toks.length - 1]);
  switch (op) {
    case "+":
      return { type: OpType.Add, val };
    case "*":
      return { type: OpType.Mul, val };
    default:
      throw new Error(`can't parse op ${s}`);
  }
}

type Monkey = {
  items: number[];
  op: Op;
  test: number;
  onTrue: number;
  onFalse: number;
};

function parseMonkey(input: string): Monkey {
  const lines = input.split("\n").map((s) => s.trim());

  const items = lines[1]
    .substring("Starting items: ".length)
    .split(", ")
    .map((s) => parseInt(s));
  const op = parseOp(lines[2]);
  const test = parseInt(lines[3].substring("Test: divisible by ".length));
  const onTrue = parseInt(
    lines[4].substring("If true: throw to monkey ".length),
  );
  const onFalse = parseInt(
    lines[5].substring("If false: throw to monkey ".length),
  );

  return { items, op, test, onTrue, onFalse };
}

const monkeys = readInput().split("\n\n").map(parseMonkey);
const inspections = new Array(monkeys.length).fill(0);

for (let round = 0; round < 20; round++) {
  for (let m = 0; m < monkeys.length; m++) {
    const monkey = monkeys[m];

    for (let itemWorry of monkey.items) {
      const op = monkey.op;
      switch (op.type) {
        case OpType.Add:
          itemWorry += op.val;
          break;

        case OpType.Mul:
          itemWorry *= op.val;
          break;

        case OpType.Square:
          itemWorry *= itemWorry;
          break;
      }

      itemWorry = Math.floor(itemWorry / 3);
      const nextMonkey = (itemWorry % monkey.test == 0)
        ? monkey.onTrue
        : monkey.onFalse;
      monkeys[nextMonkey].items.push(itemWorry);
    }

    inspections[m] += monkeys[m].items.length;
    monkeys[m].items = [];
  }
}

inspections.sort((a, b) => b - a);
console.log(inspections[0] * inspections[1]);
