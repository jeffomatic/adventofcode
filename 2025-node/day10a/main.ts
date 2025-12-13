import { stat } from 'fs';
import { example, input } from '../lib/util';

type ButtonWiring = number[];

type MachineSpec = {
  target: boolean[];
  buttons: ButtonWiring[];
  joltage: number[];
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
    target,
    buttons,
    joltage,
  };
}

function apply(state: boolean[], wiring: ButtonWiring): boolean[] {
  const next = [...state];
  for (const b of wiring) {
    next[b] = !next[b];
  }

  return next;
}

class Solver {
  memory = new Map<string, number>();
  best = Infinity;

  solve(spec: MachineSpec): number {
    const lights = new Array<boolean>(spec.target.length).fill(false);
    this.recurse(spec, lights, 0);
    return this.best;
  }

  private recurse(spec: MachineSpec, lights: boolean[], pressesSoFar: number) {
    if (pressesSoFar >= this.best) {
      return;
    }

    const stateKey = lights.join();
    if (spec.target.join() === stateKey) {
      this.best = Math.min(this.best, pressesSoFar);
      return;
    }

    const bestForThisState = this.memory.get(stateKey);
    if (bestForThisState !== undefined && bestForThisState <= pressesSoFar) {
      return;
    }

    this.memory.set(stateKey, pressesSoFar);

    for (const button of spec.buttons) {
      const next = apply(lights, button);
      const newPresses = pressesSoFar + 1;
      this.recurse(spec, next, newPresses);
    }
  }
}

// const lines = example();
const lines = input();
const machineSpecs = lines.map(parseMachineSpec);
const res = machineSpecs.reduce((accum, m) => accum + new Solver().solve(m), 0);
console.log(res);
