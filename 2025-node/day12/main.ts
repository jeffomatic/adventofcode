import { example, input } from '../lib/util';

type Shape = boolean[][];
type Region = {
  width: number;
  height: number;
  presents: number[];
};
const shapes: Shape[] = [];
const regions: Region[] = [];

const lines = example(); // 2
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
    const [width, height] = dimStr.split('x').map(Number);
    const presents = presentStr.split(' ').map(Number);
    regions.push({
      width,
      height,
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

function printRegion(r: Set<string>, width: number, height: number) {
  for (let i = 0; i < height; i++) {
    let rowStr = '';
    for (let j = 0; j < width; j++) {
      if (r.has(serializeCoord([j, i]))) {
        rowStr += '#';
      } else {
        rowStr += '.';
      }
    }
    console.log(rowStr);
  }
  console.log();
}

function rotate(present: Shape, rot: number): Shape {
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

function printShape(shape: Shape) {
  for (let i = 0; i < 3; i++) {
    let rowStr = '';
    for (let j = 0; j < 3; j++) {
      if (shape[i][j]) {
        rowStr += '#';
      } else {
        rowStr += '.';
      }
    }
    console.log(rowStr);
  }
  console.log();
}

class Solver {
  visited = new Set<string>();

  constructor(
    public width: number,
    public height: number,
  ) {}

  attemptPlace(
    present: Shape,
    x: number,
    y: number,
    rot: number,
    region: Set<string>,
  ): Set<string> | undefined {
    const rotated = rotate(present, rot);
    const newCoords: string[] = [];

    for (let i = 0; i < 3; i++) {
      const row = rotated[i];
      for (let j = 0; j < 3; j++) {
        if (!row[j]) {
          continue;
        }

        const coordStr = serializeCoord([x + j, y + i]);
        if (region.has(coordStr)) {
          return undefined;
        }

        newCoords.push(coordStr);
      }
    }

    const res = new Set(region);
    for (const coord of newCoords) {
      res.add(coord);
    }

    return res;
  }

  solve(presents: number[], occupied: Set<string>): boolean {
    // console.log(presents);

    if (presents.every((p) => p === 0)) {
      return true;
    }

    // TODO: visited probably needs to include `presents`
    const visitedKey = presents.join(',') + ':' + serializeRegion(occupied);
    if (this.visited.has(visitedKey)) {
      // console.log('dedupe');
      return false;
    }

    this.visited.add(visitedKey);

    const newPresents = [...presents];
    const presentIndex = newPresents.findIndex((n) => n > 0);
    newPresents[presentIndex] -= 1;
    const shape = shapes[presentIndex];

    for (let y = 0; y < this.height - 2; y++) {
      for (let x = 0; x < this.width - 2; x++) {
        for (let rot = 0; rot < 4; rot++) {
          const nextOccupied = this.attemptPlace(shape, x, y, rot, occupied);
          if (nextOccupied !== undefined) {
            if (this.solve(newPresents, nextOccupied)) {
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
  const solver = new Solver(region.width, region.height);
  if (solver.solve(region.presents, new Set())) {
    res += 1;
  }
}

console.log(res);
