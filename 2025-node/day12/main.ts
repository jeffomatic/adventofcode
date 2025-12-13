import { example, input } from '../lib/util';

type Shape = boolean[][];
type Region = {
  width: number;
  length: number;
  presents: number[];
};
const shapes: Shape[] = [];
const regions: Region[] = [];

const lines = example();
// const lines = input();

let parsingShapes = true;
for (let i = 0; i < lines.length; i++) {
  if (lines[i].includes('x')) {
    parsingShapes = false;
  }

  if (parsingShapes) {
    i++; // remove index

    const shape: Shape = [];
    while (lines[i] !== '') {
      shape.push(lines[i].split('').map((c) => c === '#'));
      i++;
    }

    shapes.push(shape);
  } else {
    const [dimStr, presentStr] = lines[i].split(': ');
    const [width, length] = dimStr.split('x').map(Number);
    const presents = presentStr.split(' ').map(Number);
    regions.push({
      width,
      length,
      presents,
    });
  }
}

function serializeCoord(coord: [number, number]): string {
  return `${coord[0]},${coord[1]}`;
}

function deserializeCoord(s: string): [number, number] {
  return s.split(',').map(Number) as [number, number];
}

function compareCoord(a: [number, number], b: [number, number]): number {
  if (a[0] - b[0] < 0) {
    return -1;
  }

  if (b[0] - a[0] < 0) {
    return 1;
  }

  if (a[1] - b[1] < 0) {
    return -1;
  }

  if (b[1] - a[1] < 0) {
    return 1;
  }

  return 0;
}

function serializeRegion(r: Set<string>): string {
  return [...r].map(deserializeCoord).sort(compareCoord).map(serializeCoord).join('|');
}

class Solver {
  visited = new Set<string>();

  constructor(
    public width: number,
    public length: number,
  ) {}

  rotate(present: Shape, rot: number): Shape {
    let result = structuredClone(present);

    for (let i = 0; i < rot; i++) {
      result = [
        [result[2][0], result[1][0], result[0][0]],
        [result[2][1], result[1][1], result[0][1]],
        [result[2][2], result[1][2], result[0][2]],
      ];
    }

    return result;
  }

  attemptPlace(
    present: Shape,
    x: number,
    y: number,
    rot: number,
    occupied: Set<string>,
  ): Set<string> | undefined {
    // TODO: this function

    return undefined;
  }

  solve(presents: number[], occupied: Set<string>): boolean {
    // TODO: visited probably needs to include `presents`
    if (this.visited.has(serializeRegion(occupied))) {
      return false;
    }

    this.visited.add(serializeRegion(occupied));

    if (presents.every((p) => p === 0)) {
      return true;
    }

    presents = [...presents];
    const presentIndex = presents.findIndex((n) => n > 0);
    presents[presentIndex] -= 1;
    const shape = shapes[presentIndex];

    for (let i = 0; i < this.width; i++) {
      for (let j = 0; j < this.length; i++) {
        for (let rot = 0; rot < 4; rot++) {
          const nextOccupied = this.attemptPlace(shape, i, j, rot, occupied);
          if (nextOccupied !== undefined) {
            if (this.solve(presents, nextOccupied)) {
              return true;
            }
          }
        }
      }
    }

    return false;
  }
}

let res = 0;
for (const region of regions) {
  const solver = new Solver(region.width, region.length);
  if (solver.solve(region.presents, new Set())) {
    res += 1;
  }
}

console.log(res);
