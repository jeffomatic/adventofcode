import { stat } from 'fs';
import { example, input } from '../lib/util';

type ButtonWiring = number[];

type MachineSpec = {
  lightTarget: boolean[];
  wirings: ButtonWiring[];
  joltageTarget: number[];
};

function parseMachineSpec(s: string): MachineSpec {
  let [targetStr, rest] = s.split('] ');
  targetStr = targetStr.substring(1);
  const target = targetStr.split('').map((c) => c === '#');

  let [wiringStr, joltageStr] = rest.split(' {');
  const buttons = wiringStr.split(' ').map(
    (buttonStr) =>
      buttonStr
        .substring(1, buttonStr.length - 1)
        .split(',')
        .map(Number) as ButtonWiring,
  );

  const joltage = joltageStr
    .substring(0, joltageStr.length - 1)
    .split(',')
    .map(Number);

  return {
    lightTarget: target,
    wirings: buttons,
    joltageTarget: joltage,
  };
}

function apply(state: number[], wiring: ButtonWiring): number[] {
  const next = [...state];
  for (const b of wiring) {
    next[b] -= 1;
  }

  return next;
}

class Solver {
  memory = new Map<string, number>();
  best = Infinity;

  constructor(public spec: MachineSpec) {}

  solve(): number {
    const joltage = [...this.spec.joltageTarget];
    return this.recurse(joltage, 0);
  }

  // target joltages: [0, 0, 100, 103, 105] 2 presses
  // results:
  // - [0, 0, 0, 56, 70], 5 presses
  // - [0, 0, 0, 50, 64], 10 presses
  private recurse(targetJoltages: number[], presses: number): number {
    const stateKey = targetJoltages.join(',');
    const prev = this.memory.get(stateKey);
    if (prev !== undefined && prev <= presses) {
      return prev;
    }

    if (targetJoltages.every((v) => v === 0)) {
      this.memory.set(stateKey, presses);
      return presses;
    }

    // Find the lowest nonzero joltage
    let machineIndex = 0;
    let lowestTarget = Infinity;
    for (let i = 0; i < targetJoltages.length; i++) {
      if (targetJoltages[i] > 0 && targetJoltages[i] < lowestTarget) {
        machineIndex = i;
        lowestTarget = targetJoltages[i];
      }
    }

    const zeroIndexes: number[] = [];
    for (let i = 0; i < targetJoltages.length; i++) {
      if (targetJoltages[i] === 0) {
        zeroIndexes.push(i);
      }
    }

    const buttons = this.spec.wirings
      .filter((w) => w.includes(machineIndex) && zeroIndexes.every((index) => !w.includes(index)))
      .sort((b1, b2) => b2.length - b1.length);

    let best = Infinity;
    for (const button of buttons) {
      const newTarget = apply(targetJoltages, button);
      if (!newTarget.every((v) => v >= 0)) {
        continue;
      }

      const res = this.recurse(newTarget, presses + 1);
      best = Math.min(best, res);
    }

    if (best !== Infinity) {
      this.memory.set(stateKey, best);
    }

    return best;
  }
}

/*
solve(spec, joltages) -> {number[], presses}[]
*/

// const lines = example(); // 33
const lines = input();
const machineSpecs = lines.map(parseMachineSpec);
const res = machineSpecs.reduce((accum, m) => {
  const res = new Solver(m).solve();
  console.log(m, res);

  return accum + res;
}, 0);
console.log(res);
