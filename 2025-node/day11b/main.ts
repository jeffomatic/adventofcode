import { example, input } from '../lib/util';

// const lines = example(); // 5
const lines = input();
const graph: Record<string, string[]> = {};
for (const line of lines) {
  const [server, right] = line.split(': ');
  const edges = right.split(' ');
  graph[server] = edges;
}

class Solver {
  cache = new Map<string, number>();

  numPaths(from: string, to: string, exclude?: string): number {
    const cached = this.cache.get(from);
    if (cached !== undefined) {
      return cached;
    }

    if (graph[from].includes(to)) {
      return 1;
    }

    const edges = graph[from].filter((edge) => edge !== 'out' && edge !== exclude);
    const res = edges.reduce((accum, edge) => accum + this.numPaths(edge, to), 0);
    this.cache.set(from, res);

    return res;
  }
}

const svrToFft = new Solver().numPaths('svr', 'fft', 'dac');
const svrToDac = new Solver().numPaths('svr', 'dac', 'fft');
const fftToDac = new Solver().numPaths('fft', 'dac');
const dacToFft = new Solver().numPaths('dac', 'fft');
const dacToOut = new Solver().numPaths('dac', 'out', 'fft');
const fftToOut = new Solver().numPaths('fft', 'out', 'dac');

const res = svrToFft * fftToDac * dacToOut + svrToDac * dacToFft * fftToOut;

console.log(res);
