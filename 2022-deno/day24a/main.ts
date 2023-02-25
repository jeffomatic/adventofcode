import { readInputLines } from "../common.ts";
import * as vec2 from "../vec2.ts";
type Vec2 = vec2.Vec2;

enum Dir {
  N,
  S,
  W,
  E,
}

function wrapMove(src: Vec2, dir: Dir, dimensions: Vec2): Vec2 {
  const nextPos = vec2.copy(src);

  switch (dir) {
    case Dir.N:
      nextPos[0] -= 1;
      if (nextPos[0] < 0) {
        nextPos[0] = dimensions[0] - 1;
      }
      return nextPos;

    case Dir.S:
      nextPos[0] += 1;
      if (nextPos[0] >= dimensions[0]) {
        nextPos[0] = 0;
      }
      return nextPos;

    case Dir.W:
      nextPos[1] -= 1;
      if (nextPos[1] < 0) {
        nextPos[1] = dimensions[1] - 1;
      }
      return nextPos;

    case Dir.E:
      nextPos[1] += 1;
      if (nextPos[1] >= dimensions[1]) {
        nextPos[1] = 0;
      }
      return nextPos;
  }
}

function occupancyKey(pos: Vec2, stride: number): number {
  return pos[0] * stride + pos[1];
}

class State {
  dimensions: Vec2;
  dirs: Dir[];
  positions: Vec2[];
  occupied: Uint8Array;

  constructor(dirs: Dir[], positions: Vec2[], dimensions: Vec2) {
    this.dimensions = dimensions;
    this.dirs = dirs;
    this.positions = positions;

    // This is the slowest aspect of state generation, so use an efficient array.
    this.occupied = new Uint8Array(dimensions[0] * dimensions[1]);
    for (const p of positions) {
      this.occupied[occupancyKey(p, dimensions[1])] = 1;
    }
  }

  isOpen(pos: Vec2): boolean {
    return this.occupied[occupancyKey(pos, this.dimensions[1])] != 1;
  }

  print(): string {
    const s: string[][] = [];
    const [h, w] = this.dimensions;
    for (let i = 0; i < h; i++) {
      const row: string[] = [];
      s.push(row);

      for (let j = 0; j < w; j++) {
        row.push(".");
      }
    }

    for (let i = 0; i < this.dirs.length; i++) {
      const pos = this.positions[i];

      switch (this.dirs[i]) {
        case Dir.N:
          s[pos[0]][pos[1]] = "^";
          break;
        case Dir.S:
          s[pos[0]][pos[1]] = "v";
          break;
        case Dir.W:
          s[pos[0]][pos[1]] = "<";
          break;
        case Dir.E:
          s[pos[0]][pos[1]] = ">";
          break;
      }
    }

    return s.map((row) => row.join("")).join("\n");
  }

  next(): State {
    const positions: Vec2[] = new Array(this.positions.length);
    for (let i = 0; i < this.positions.length; i++) {
      positions[i] = wrapMove(this.positions[i], this.dirs[i], this.dimensions);
    }

    return new State(this.dirs, positions, this.dimensions);
  }

  static parse(input: string[]): State {
    // Remove boundaries
    input = input.slice(1, input.length - 1).map((row) =>
      row.slice(1, row.length - 1)
    );

    const h = input.length;
    const w = input[0].length;

    const dirs: Dir[] = [];
    const positions: Vec2[] = [];
    for (let i = 0; i < h; i++) {
      for (let j = 0; j < w; j++) {
        const pos = vec2.make(i, j);

        switch (input[i][j]) {
          case "^":
            dirs.push(Dir.N);
            positions.push(pos);
            break;

          case "v":
            dirs.push(Dir.S);
            positions.push(pos);
            break;

          case "<":
            dirs.push(Dir.W);
            positions.push(pos);
            break;

          case ">":
            dirs.push(Dir.E);
            positions.push(pos);
            break;
        }
      }
    }

    return new State(dirs, positions, vec2.make(h, w));
  }
}

type Task = {
  stateIndex: number;
  pos: Vec2;
  time: number;
};

function taskKey(t: Task): string {
  return `${t.stateIndex}:${t.pos[0]}.${t.pos[1]}`;
}

function search(states: State[]): number {
  const period = states.length;
  const dimensions = states[0].dimensions;
  const goal = vec2.make(dimensions[0], dimensions[1] - 1);
  const initial: Task = {
    stateIndex: 0,
    pos: vec2.make(-1, 0),
    time: 0,
  };

  const visited = new Set<string>();
  visited.add(taskKey(initial));

  const q: Task[] = [initial];
  while (q.length > 0) {
    const task = q.shift()!;

    const candidates: Task[] = [];
    const nextStateIndex = (task.stateIndex + 1) % period;

    // Moves
    for (const d of [Dir.N, Dir.S, Dir.W, Dir.E]) {
      const nextPos = vec2.copy(task.pos);
      switch (d) {
        case Dir.N:
          nextPos[0] -= 1;
          break;

        case Dir.S:
          nextPos[0] += 1;
          break;

        case Dir.W:
          nextPos[1] -= 1;
          break;

        case Dir.E:
          nextPos[1] += 1;
          break;
      }

      // boundary check
      if (!vec2.equal(nextPos, goal)) {
        if (
          nextPos[0] < 0 || dimensions[0] <= nextPos[0] ||
          nextPos[1] < 0 || dimensions[1] <= nextPos[1]
        ) {
          continue;
        }
      }

      candidates.push({
        stateIndex: nextStateIndex,
        pos: nextPos,
        time: task.time + 1,
      });
    }

    // Wait
    candidates.push({
      stateIndex: nextStateIndex,
      pos: task.pos,
      time: task.time + 1,
    });

    // Validate each task and push onto the queue
    for (const t of candidates) {
      const key = taskKey(t);
      if (visited.has(key)) {
        continue;
      }

      const state = states[t.stateIndex];
      if (!state.isOpen(t.pos)) {
        continue;
      }

      if (vec2.equal(t.pos, goal)) {
        return t.time;
      }

      q.push(t);
      visited.add(key);
    }
  }

  throw new Error("could not find solution");
}

function getCycle(initialState: State): State[] {
  const k = initial.print();
  const res: State[] = [initialState];
  while (true) {
    const next = res[res.length - 1].next();
    if (next.print() == k) {
      return res;
    }
    res.push(next);
  }
}

const initial = State.parse(readInputLines());
const states = getCycle(initial);
const res = search(states);
console.log(res);
