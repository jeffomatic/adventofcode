import { example, input } from '../lib/util';

// const lines = example(); // 5
const lines = input();
const graph: Record<string, string[]> = {};
for (const line of lines) {
  const [server, right] = line.split(': ');
  const edges = right.split(' ');
  graph[server] = edges;
}

const cache = new Map<string, number>();

function numPaths(from: string): number {
  const cached = cache.get(from);
  if (cached !== undefined) {
    return cached;
  }

  if (graph[from].includes('out')) {
    return 1;
  }

  const res = graph[from].reduce((accum, edge) => accum + numPaths(edge), 0);
  cache.set(from, res);
  return res;
}

const res = numPaths('you');

console.log(res);
