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
    next[b] += 1;
  }

  return next;
}

class Solver {
  memory = new Map<string, number>();
  best = Infinity;

  solve(spec: MachineSpec): number {
    const joltage = new Array<number>(spec.joltageTarget.length).fill(0);
    this.recurse(spec, joltage, 0);
    return this.best;
  }

  private recurse(spec: MachineSpec, joltage: number[], pressesSoFar: number) {
    if (pressesSoFar >= this.best) {
      return;
    }

    for (let i = 0; i < joltage.length; i++) {
      if (spec.joltageTarget[i] < joltage[i]) {
        return;
      }
    }

    const stateKey = joltage.join();
    if (spec.joltageTarget.join() === stateKey) {
      this.best = Math.min(this.best, pressesSoFar);
      return;
    }

    const bestForThisState = this.memory.get(stateKey);
    if (bestForThisState !== undefined && bestForThisState <= pressesSoFar) {
      return;
    }

    this.memory.set(stateKey, pressesSoFar);

    for (const button of spec.wirings) {
      const next = apply(joltage, button);
      const newPresses = pressesSoFar + 1;
      this.recurse(spec, next, newPresses);
    }
  }
}

// const lines = example(); // 33
const lines = input();
const machineSpecs = lines.map(parseMachineSpec);
const res = machineSpecs.reduce((accum, m) => {
  console.log(m);
  const res = new Solver().solve(m);
  console.log(res);
  return accum + res;
}, 0);
console.log(res);
