import { readInputLines } from "../common.ts";

type Valve = {
  name: string;
  flow: number;
  neighbors: string[];
};

function parseValve(s: string): Valve {
  const regexp =
    /Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)/;
  const matches = s.match(regexp)!;
  const name = matches[1];
  const flow = parseInt(matches[2]);
  const neighbors = matches[3].split(", ");
  return { name, flow, neighbors };
}

type OpenValve = [string, number];

class State {
  pos: string;
  openValves: OpenValve[];

  constructor(pos: string, valves: OpenValve[]) {
    this.pos = pos;
    this.openValves = valves;
  }

  move(dst: string): State {
    return new State(dst, [...this.openValves]);
  }

  open(valve: string, timeRemaining: number): State {
    const valves: OpenValve[] = [...this.openValves, [valve, timeRemaining]];
    valves.sort(State.cmpOpenValve);
    return new State(this.pos, valves);
  }

  isValveOpen(valve: string): boolean {
    return this.openValves.findIndex((v) => v[0] == valve) >= 0;
  }

  key(): string {
    return this.pos +
      "|" +
      this.openValves.map(State.valveKey).join(".");
  }

  score(scores: Record<string, number>): number {
    return this.openValves
      .map((v) => scores[v[0]] * v[1])
      .reduce((memo, s) => memo + s);
  }

  static valveKey([name, timeRemaining]: OpenValve): string {
    return `${name}:${timeRemaining}`;
  }

  static cmpOpenValve(a: OpenValve, b: OpenValve): number {
    const dt = b[1] - a[1];
    if (dt != 0) {
      return dt;
    }

    return a[0].localeCompare(b[0]);
  }
}

type SearchNode = [number, State]; // [time, State]
type Queue = SearchNode[];

function enqueue(q: Queue, node: SearchNode) {
  q.push(node);

  for (let i = q.length - 1; i > 0; i--) {
    const left = q[i - 1];
    const right = q[i];
    if (left[0] <= right[0]) {
      break;
    }

    q[i - 1] = right;
    q[i] = left;
  }
}

const valves = readInputLines(import.meta.url).map(parseValve);
const scores: Record<string, number> = valves.reduce(
  (memo, node) => {
    memo[node.name] = node.flow;
    return memo;
  },
  {} as Record<string, number>,
);
const neighbors: Record<string, string[]> = valves.reduce(
  (memo, node) => {
    memo[node.name] = node.neighbors;
    return memo;
  },
  {} as Record<string, string[]>,
);
const nonzeroScores = valves.map((v) => v.flow).filter((f) => f > 0).length;

const baseState: State = new State("AA", []);
const q: Queue = [[0, baseState]];
const visited = new Set<string>([baseState.key()]);
const maxTime = 30;
let best = 0;

while (q.length > 0) {
  const [time, state] = q.shift()!;
  const options: State[] = [];

  // Option 1: spend a minute to open the current valve
  if (scores[state.pos] > 0 && !state.isValveOpen(state.pos)) {
    options.push(state.open(state.pos, maxTime - time - 1));
  }

  // Option 2+: move to a neighbor
  for (const n of neighbors[state.pos]) {
    options.push(state.move(n));
  }

  for (const opt of options) {
    if (visited.has(opt.key())) {
      continue;
    }

    visited.add(opt.key());

    if (opt.openValves.length == nonzeroScores || time == maxTime) {
      best = Math.max(best, opt.score(scores));
    } else {
      enqueue(q, [time + 1, opt]);
    }
  }
}

console.log(best);
