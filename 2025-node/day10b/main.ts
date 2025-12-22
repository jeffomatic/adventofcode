import { example, input } from '../lib/util';
import * as jslpsolver from '@bygdle/javascript-lp-solver';

type ButtonWiring = number[];

type MachineSpec = {
  buttons: ButtonWiring[];
  targets: number[];
};

function parseMachineSpec(s: string): MachineSpec {
  let [, rest] = s.split('] ');
  let [buttonsStr, targetsStr] = rest.split(' {');
  const buttons = buttonsStr.split(' ').map(
    (s) =>
      s
        .substring(1, s.length - 1)
        .split(',')
        .map(Number) as ButtonWiring,
  );

  const targets = targetsStr
    .substring(0, targetsStr.length - 1)
    .split(',')
    .map(Number);

  return { buttons, targets };
}

// In the language of jslpsolver, each of our targets is a "constraint". Each button is a "variable"
// that will contribute a certain amount to each constraint.
type SolverConstraint = { equal: number };
type SolverVariable = { cost: 1 } & Record<string, number>;

function targetConstraintName(index: number): string {
  return `t${index}`;
}

function buttonVarName(index: number): string {
  return `b${index}`;
}

function solve(spec: MachineSpec): number {
  // Each spec really just describes a linear equation Ax = B:
  //
  // - Each column of matrix A is a button wiring, i.e. a vector of 0s or 1s where 1 means the
  //   button increments the light
  // - There are as many columns as there are buttons
  // - Each element of x is how many times you press each button
  // - B is the target value
  //
  // So we are solving for x, and then adding up all the elements.
  //
  // Finding a library that does integer linear solving is rough in NodeJS...the best I could find
  // is an ancient library called javascript-lp-solver, which is broken in the base repo, so I had
  // to use some rando's fork.

  const constraints: Record<string, SolverConstraint> = {};
  for (let i = 0; i < spec.targets.length; i++) {
    constraints[targetConstraintName(i)] = { equal: spec.targets[i] };
  }

  const variables: Record<string, SolverVariable> = {};
  const ints: Record<string, 1> = {};
  for (let i = 0; i < spec.buttons.length; i++) {
    const sv: SolverVariable = { cost: 1 };

    // Indicate which joltage targets this button will increment.
    for (const joltage of spec.buttons[i]) {
      sv[targetConstraintName(joltage)] = 1;
    }

    const k = buttonVarName(i);
    variables[k] = sv;
    ints[k] = 1; // indicate that this variable must yield an integer
  }

  const solution = jslpsolver.Solve({
    optimize: 'cost',
    opType: 'min',
    constraints,
    variables,
    ints,
  });

  if (!solution.feasible) {
    throw new Error(`not feasible: ${JSON.stringify(spec)}`);
  }

  let res = 0;
  for (let i = 0; i < spec.buttons.length; i++) {
    res += solution[buttonVarName(i)] ?? 0;
  }

  return res;
}

// const lines = example(); // 33
const lines = input();
const machineSpecs = lines.map(parseMachineSpec);
const res = machineSpecs.reduce((accum, spec) => accum + solve(spec), 0);
console.log(res);
