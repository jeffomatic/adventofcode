import { readInput } from "../common.ts";
import * as vec2 from "../vec2.ts";
type Vec2 = vec2.Vec2;

type Range = [number, number];

type Map = {
  rows: Range[];
  cols: Range[];
  walls: Set<string>;
};

function makeMap(): Map {
  return {
    rows: [],
    cols: [],
    walls: new Set<string>(),
  };
}

enum Turn {
  Left,
  Right,
}

enum StepType {
  Move,
  Turn,
}

type TurnStep = { type: StepType.Turn; turn: Turn };
type MoveStep = { type: StepType.Move; steps: number };
type Step = TurnStep | MoveStep;

enum Dir {
  N,
  S,
  W,
  E,
}

function turn(from: Dir, turn: Turn): Dir {
  switch (from) {
    case Dir.N:
      return turn == Turn.Left ? Dir.W : Dir.E;
    case Dir.S:
      return turn == Turn.Left ? Dir.E : Dir.W;
    case Dir.W:
      return turn == Turn.Left ? Dir.S : Dir.N;
    case Dir.E:
      return turn == Turn.Left ? Dir.N : Dir.S;
  }
}

function dirScore(d: Dir): number {
  switch (d) {
    case Dir.N:
      return 3;
    case Dir.S:
      return 1;
    case Dir.W:
      return 2;
    case Dir.E:
      return 0;
  }
}

function dirIncr(d: Dir): Vec2 {
  switch (d) {
    case Dir.N:
      return [-1, 0];
    case Dir.S:
      return [1, 0];
    case Dir.W:
      return [0, -1];
    case Dir.E:
      return [0, 1];
  }
}

function parseMap(input: string): Map {
  const mapLines = input.split("\n");
  const rowCount = mapLines.length;
  const colCount = mapLines.reduce(
    (memo, line) => Math.max(memo, line.length),
    0,
  );

  const map = makeMap();
  for (let i = 0; i < rowCount; i++) {
    const row = mapLines[i];
    let start = undefined;
    let end = undefined;

    for (let j = 0; j < colCount; j++) {
      if (row[j] == "#") {
        map.walls.add(vec2.key([i, j]));
      }

      if (start == undefined) {
        if (row[j] == "#" || row[j] == ".") {
          start = j;
        }

        continue;
      }

      if (row[j + 1] == undefined) {
        end = j;
        break;
      }
    }

    map.rows.push([start!, end!]);
  }

  for (let j = 0; j < colCount; j++) {
    let start = undefined;
    let end = undefined;

    for (let i = 0; i < rowCount; i++) {
      if (start == undefined) {
        if (mapLines[i][j] == "#" || mapLines[i][j] == ".") {
          start = i;
        }

        continue;
      }

      if (
        mapLines[i + 1] == undefined ||
        mapLines[i + 1][j] == " " ||
        mapLines[i + 1][j] == undefined
      ) {
        end = i;
        break;
      }
    }

    map.cols.push([start!, end!]);
  }

  return map;
}

function parseSteps(input: string): Step[] {
  const res: Step[] = [];

  let pos = 0;
  while (pos < input.length) {
    switch (input[pos]) {
      case "L":
        res.push({ type: StepType.Turn, turn: Turn.Left });
        pos += 1;
        break;

      case "R":
        res.push({ type: StepType.Turn, turn: Turn.Right });
        pos += 1;
        break;

      default:
        {
          let numStr = "";
          const zero = "0".charCodeAt(0);
          const nine = "9".charCodeAt(0);

          while (pos < input.length) {
            const c = input.charCodeAt(pos);
            if (c < zero || nine < c) {
              break;
            }

            numStr += input[pos];
            pos += 1;
          }

          res.push({ type: StepType.Move, steps: parseInt(numStr) });
        }
        break;
    }
  }

  return res;
}

type State = [Vec2, Dir];

function doStep(state: State, step: Step, map: Map): State {
  switch (step.type) {
    case StepType.Turn:
      return doTurn(state, step.turn);

    case StepType.Move:
      return doMove(state, step.steps, map);
  }
}

function doTurn([pos, dir]: State, t: Turn): State {
  return [pos, turn(dir, t)];
}

function move(pos: Vec2, dir: Dir, map: Map): Vec2 {
  let nextPos = vec2.add(pos, dirIncr(dir));

  // wraparound check
  let [row, col] = nextPos;
  switch (dir) {
    case Dir.N:
      if (row < map.cols[col][0]) {
        row = map.cols[col][1];
      }
      break;

    case Dir.S:
      if (row > map.cols[col][1]) {
        row = map.cols[col][0];
      }
      break;

    case Dir.W:
      if (col < map.rows[row][0]) {
        col = map.rows[row][1];
      }
      break;

    case Dir.E:
      if (col > map.rows[row][1]) {
        col = map.rows[row][0];
      }
      break;
  }

  nextPos = [row, col];

  // wall check
  if (map.walls.has(vec2.key(nextPos))) {
    return pos;
  }

  return nextPos;
}

const debugMoves: State[] = [];

function doMove([pos, dir]: State, steps: number, map: Map): State {
  for (let i = 0; i < steps; i++) {
    const nextPos = move(pos, dir, map);
    if (vec2.equal(pos, nextPos)) {
      break;
    }

    pos = nextPos;
    debugMoves.push([pos, dir]);
  }

  return [pos, dir];
}

const [mapInput, stepsInput] = readInput().split("\n\n");
const map = parseMap(mapInput);
const steps = parseSteps(stepsInput);

const initial: State = [[0, map.rows[0][0]], Dir.E];
const [[row, col], dir] = steps.reduce(
  (state, step) => doStep(state, step, map),
  initial,
);
const res = 1000 * (row + 1) + 4 * (col + 1) + dirScore(dir);
console.log(res);
