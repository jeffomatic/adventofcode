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

function move(pos: Vec2, dir: Dir, map: Map): [Vec2, Dir] {
  const [prevRow, prevCol] = pos;
  let [nextRow, nextCol] = vec2.add(pos, dirIncr(dir));
  let nextDir = dir;

  // wraparound check
  switch (dir) {
    case Dir.N:
      if (nextRow < map.cols[nextCol][0]) {
        if (prevCol < 50) {
          nextRow = 50 + prevCol;
          nextCol = 50;
          nextDir = Dir.E;
        } else if (prevCol < 100) {
          nextRow = 150 + (prevCol - 50);
          nextCol = 0;
          nextDir = Dir.E;
        } else {
          nextRow = 199;
          nextCol = prevCol - 100;
          // no dir change
        }
      }
      break;

    case Dir.S:
      if (nextRow > map.cols[nextCol][1]) {
        if (prevCol < 50) {
          nextRow = 0;
          nextCol = 100 + prevCol;
          // no dir change
        } else if (prevCol < 100) {
          nextRow = 150 + (prevCol - 50);
          nextCol = 49;
          nextDir = Dir.W;
        } else {
          nextRow = 50 + (prevCol - 100);
          nextCol = 99;
          nextDir = Dir.W;
        }
      }
      break;

    case Dir.W:
      if (nextCol < map.rows[nextRow][0]) {
        if (prevRow < 50) {
          nextRow = 149 - prevRow;
          nextCol = 0;
          nextDir = Dir.E;
        } else if (prevRow < 100) {
          nextRow = 100;
          nextCol = prevRow - 50;
          nextDir = Dir.S;
        } else if (prevRow < 150) {
          nextRow = 149 - prevRow;
          nextCol = 50;
          nextDir = Dir.E;
        } else {
          nextRow = 0;
          nextCol = 50 + (prevRow - 150);
          nextDir = Dir.S;
        }
      }
      break;

    case Dir.E:
      if (nextCol > map.rows[nextRow][1]) {
        if (prevRow < 50) {
          nextRow = 149 - prevRow;
          nextCol = 99;
          nextDir = Dir.W;
        } else if (prevRow < 100) {
          nextRow = 49;
          nextCol = 100 + (prevRow - 50);
          nextDir = Dir.N;
        } else if (prevRow < 150) {
          nextRow = 149 - prevRow;
          nextCol = 149;
          nextDir = Dir.W;
        } else {
          nextRow = 149;
          nextCol = 50 + (prevRow - 150);
          nextDir = Dir.N;
        }
      }
      break;
  }

  // wall check
  const nextPos: Vec2 = [nextRow, nextCol];
  if (map.walls.has(vec2.key(nextPos))) {
    return [pos, dir];
  }

  return [nextPos, nextDir];
}

function doMove([pos, dir]: State, steps: number, map: Map): State {
  for (let i = 0; i < steps; i++) {
    const [nextPos, nextDir] = move(pos, dir, map);
    if (vec2.equal(pos, nextPos)) {
      break;
    }

    pos = nextPos;
    dir = nextDir;
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
