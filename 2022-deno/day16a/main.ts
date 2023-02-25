import { difference, readInputLines } from "../common.ts";

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

const valves = readInputLines(import.meta.url).map(parseValve);
const maxTime = 30;

const valvesByName = valves.reduce((memo, v) => {
  memo[v.name] = v;
  return memo;
}, {} as Record<string, Valve>);
const targetValves = new Set<string>(
  valves.filter((v) => v.flow > 0).map((v) => v.name),
);

function edgeName(a: string, b: string): string {
  return `${a}.${b}`;
}

function makeEdgeList(): Record<string, number> {
  const res: Record<string, number> = {};
  for (const v of valves.filter((v) => v.name == "AA" || v.flow > 0)) {
    const visited = new Set<string>([v.name]);
    const q: [string, number][] = [[v.name, 0]];

    while (q.length > 0) {
      const [name, dist] = q.shift()!;
      const cur = valvesByName[name];

      if (cur.flow > 0 && dist > 0) {
        res[edgeName(v.name, cur.name)] = dist;
      }

      for (const n of cur.neighbors) {
        if (visited.has(n)) {
          continue;
        }

        visited.add(n);
        q.push([n, dist + 1]);
      }
    }
  }

  return res;
}

const edgeList = makeEdgeList();

type Sequence = string[];

function sequenceKey(seq: Sequence): string {
  return seq.join("");
}

class SearchCache {
  public time(seq: Sequence): number {
    return this.get(seq)[0];
  }

  public score(seq: Sequence): number {
    return this.get(seq)[1];
  }

  private cache: Record<string, [number, number]> = {};

  private get(seq: Sequence): [number, number] {
    return (this.cache[this.key(seq)] ?? this.fill(seq));
  }

  private fill(seq: Sequence): [number, number] {
    let time = 0;
    let score = 0;
    let [cur, ...rest] = seq;
    const visited = new Set<string>(["AA"]);

    for (const n of rest) {
      if (!visited.has(n)) {
        visited.add(n);
        time += edgeList[edgeName(cur, n)] + 1;
        score += valvesByName[n].flow * (maxTime - time);
      } else {
        time += edgeList[edgeName(cur, n)];
      }

      cur = n;
    }

    const entry: [number, number] = [time, score];
    this.cache[this.key(seq)] = entry;
    return entry;
  }

  private key(path: string[]) {
    return path.join("");
  }
}

const searchCache = new SearchCache();

class SearchQueue {
  private items: Sequence[] = [];
  private visited: Set<string> = new Set<string>();

  size(): number {
    return this.items.length;
  }

  pop(): Sequence {
    return this.items.shift()!;
  }

  enqueue(seq: Sequence) {
    const k = sequenceKey(seq);
    if (this.visited.has(k)) {
      return;
    }

    this.visited.add(k);

    this.items.push(seq);
    for (let i = this.items.length - 1; i > 0; i--) {
      const left = this.items[i - 1];
      const right = this.items[i];
      if (searchCache.time(left) <= searchCache.time(right)) {
        break;
      }

      this.items[i - 1] = right;
      this.items[i] = left;
    }
  }
}

function search(): number {
  const q = new SearchQueue();
  q.enqueue(["AA"]);
  const bestAtTime: Record<number, number> = {};

  while (q.size() > 0) {
    const seq = q.pop();

    const options = difference(targetValves, new Set(seq));
    for (const opt of options) {
      const next = [...seq, opt];
      const time = searchCache.time(next);
      if (time > maxTime) {
        continue;
      }

      // Greedily eliminate next options. If the next option does not provide
      // at least as good a score as the best we've gotten by this time, then
      // ignore it. It's not obvious to me that this should be true in general,
      // but it seems to work for this problem.
      const best = bestAtTime[time] ?? 0;
      const score = searchCache.score(next);
      if (score >= best) {
        bestAtTime[time] = score;
        q.enqueue(next);
      }
    }
  }

  return Object.values(bestAtTime).sort((a, b) => b - a)[0];
}

console.log(search());
