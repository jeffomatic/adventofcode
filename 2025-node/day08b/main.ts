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

// const lines = example(); // 25272
const lines = input();
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

for (const pair of pairs) {
  const [ca, cb] = pair.items;
  const circuitA = coordToCircuit.get(ca)!;
  const circuitB = coordToCircuit.get(cb)!;
  if (circuitA === circuitB) {
    // Already in circuit
    continue;
  }

  if (circuitA.size + circuitB.size == coords.length) {
    console.log(coords[ca][0] * coords[cb][0]);
    break;
  }

  for (const coord of circuitB) {
    circuitA.add(coord);
    coordToCircuit.set(coord, circuitA);
  }

  circuitB.clear();
}
