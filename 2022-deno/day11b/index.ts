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
  startingItems: number[];
  congruentItems: number[][];
  op: Op;
  modulus: number;
  onTrue: number;
  onFalse: number;
};

function parseMonkey(input: string): Monkey {
  const lines = input.split("\n").map((s) => s.trim());

  const startingItems = lines[1]
    .substring("Starting items: ".length)
    .split(", ")
    .map((s) => parseInt(s));
  const congruentItems: number[][] = [];
  const op = parseOp(lines[2]);
  const modulus = parseInt(lines[3].substring("Test: divisible by ".length));
  const onTrue = parseInt(
    lines[4].substring("If true: throw to monkey ".length),
  );
  const onFalse = parseInt(
    lines[5].substring("If false: throw to monkey ".length),
  );

  return { startingItems, congruentItems, op, modulus, onTrue, onFalse };
}

const monkeys = readInput().split("\n\n").map(parseMonkey);
const moduli = monkeys.map((m) => m.modulus);
for (const monkey of monkeys) {
  monkey.congruentItems = monkey.startingItems.map((n) =>
    moduli.map((mod) => n % mod)
  );
}

const inspections = new Array(monkeys.length).fill(0);
const numRounds = 10000;
for (let round = 0; round < numRounds; round++) {
  for (let m = 0; m < monkeys.length; m++) {
    const monkey = monkeys[m];

    for (const worry of monkey.congruentItems) {
      const op = monkey.op;
      switch (op.type) {
        case OpType.Add:
          for (let i = 0; i < worry.length; i++) {
            worry[i] = (worry[i] + op.val) % moduli[i];
          }
          break;

        case OpType.Mul:
          for (let i = 0; i < worry.length; i++) {
            worry[i] = (worry[i] * op.val) % moduli[i];
          }
          break;

        case OpType.Square:
          for (let i = 0; i < worry.length; i++) {
            worry[i] = (worry[i] * worry[i]) % moduli[i];
          }
          break;
      }

      const nextMonkey = (worry[m] == 0) ? monkey.onTrue : monkey.onFalse;
      monkeys[nextMonkey].congruentItems.push(worry);
    }

    inspections[m] += monkeys[m].congruentItems.length;
    monkeys[m].congruentItems = [];
  }
}

inspections.sort((a, b) => b - a);
console.log(inspections[0] * inspections[1]);
