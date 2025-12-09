import { example, input } from '../lib/util';

type Coord = [number, number, number];

type Pair = {
  items: [number, number];
  dist: number;
};

function distance(a: Coord, b: Coord): number {
  const dx = a[0] - b[0];
  const dy = a[1] - b[1];
  const dz = a[2] - b[2];
  return dx * dx + dy * dy + dz * dz;
}

// const lines = example();
// const iterations = 10;
const lines = input();
const iterations = 1000;
const coords: Coord[] = lines.map((s) => s.split(',').map((n) => parseInt(n)) as Coord);

const circuits: Set<number>[] = [];
const coordToCircuit = new Map<number, Set<number>>();
for (let i = 0; i < coords.length; i++) {
  const circuit = new Set([i]);
  circuits.push(circuit);
  coordToCircuit.set(i, circuit);
}

const pairs: Pair[] = [];
for (let i = 0; i < coords.length - 1; i++) {
  for (let j = i + 1; j < coords.length; j++) {
    const a = coords[i];
    const b = coords[j];
    pairs.push({
      items: [i, j],
      dist: distance(a, b),
    });
  }
}

pairs.sort((a, b) => {
  return a.dist - b.dist;
});

for (const pair of pairs.slice(0, iterations)) {
  const [ca, cb] = pair.items;
  const circuitA = coordToCircuit.get(ca)!;
  const circuitB = coordToCircuit.get(cb)!;
  if (circuitA === circuitB) {
    // Already in circuit
    continue;
  }

  for (const coord of circuitB) {
    circuitA.add(coord);
    coordToCircuit.set(coord, circuitA);
  }

  circuitB.clear();
}

const [a, b, c] = circuits
  .filter((c) => c.size > 0)
  .map((c) => c.size)
  .sort((a, b) => a - b)
  .reverse();
const res = a * b * c;

console.log(res);
