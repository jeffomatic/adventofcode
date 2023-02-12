import { difference } from "../common.ts";
import { compactEdgeList, edgeKey, Valve } from "./graph.ts";

export type Sequence = string[];

export function sequenceKey(seq: Sequence): string {
  return seq.join("");
}

export class SearchCache {
  private valvesByName: Record<string, Valve>;
  private maxTime: number;
  private edgeList: Record<string, number>;

  constructor(valves: Valve[], start: string, maxTime: number) {
    this.valvesByName = valves.reduce((memo, v) => {
      memo[v.name] = v;
      return memo;
    }, {} as Record<string, Valve>);
    this.maxTime = maxTime;
    this.edgeList = compactEdgeList(this.valvesByName, start);
  }

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
        time += this.edgeList[edgeKey([cur, n])] + 1;
        score += this.valvesByName[n].flow * (this.maxTime - time);
      } else {
        time += this.edgeList[edgeKey([cur, n])];
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

class SearchQueue {
  private items: Sequence[] = [];
  private visited: Set<string> = new Set<string>();
  private searchCache: SearchCache;

  constructor(searchCache: SearchCache) {
    this.searchCache = searchCache;
  }

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
      if (this.searchCache.time(left) <= this.searchCache.time(right)) {
        break;
      }

      this.items[i - 1] = right;
      this.items[i] = left;
    }
  }
}

export function search(
  valves: Valve[],
  start: string,
  maxTime: number,
): number {
  const targetValves = new Set<string>(
    Object.values(valves).filter((v) => v.flow > 0).map((v) => v.name),
  );
  const cache = new SearchCache(valves, start, maxTime);
  const q = new SearchQueue(cache);
  q.enqueue(["AA"]);
  const bestAtTime: Record<number, number> = {};

  while (q.size() > 0) {
    const seq = q.pop();

    const options = difference(targetValves, new Set(seq));
    for (const opt of options) {
      const next = [...seq, opt];
      const time = cache.time(next);
      if (time > maxTime) {
        continue;
      }

      // Greedily eliminate next options. If the next option does not provide
      // at least as good a score as the best we've gotten by this time, then
      // ignore it. It's not obvious to me that this should be true in general,
      // but it seems to work for this problem.
      const best = bestAtTime[time] ?? 0;
      const score = cache.score(next);
      if (score >= best) {
        bestAtTime[time] = score;
        q.enqueue(next);
      }
    }
  }

  return Object.values(bestAtTime).sort((a, b) => b - a)[0];
}
