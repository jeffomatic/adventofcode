import { example, input } from '../lib/util';

type Shape = boolean[][];

type Region = {
  width: number;
  height: number;
  presents: number[];
};

const shapes: Shape[] = [];
const regions: Region[] = [];

// const lines = example(); // 2
const lines = input();

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

function shapeWeight(shape: Shape) {
  let weight = 0;
  for (const row of shape) {
    for (const cell of row) {
      weight += cell ? 1 : 0;
    }
  }
  return weight;
}

let naiveFits = 0;
let exactFits = 0;
for (const r of regions) {
  const area = r.width * r.height;
  const naive = r.presents.reduce((accum, count) => accum + 9 * count, 0);
  const exact = r.presents.reduce(
    (accum, count, index) => accum + shapeWeight(shapes[index]) * count,
    0,
  );

  if (naive <= area) {
    naiveFits += 1;
  }

  if (exact <= area) {
    exactFits += 1;
  }

  console.log(
    `${r.width}x${r.height}: area: ${area}, naive ${naive}${naive <= area ? ' (fits)' : ''}, exact ${exact}${exact <= area ? ' (fits)' : ''}`,
  );
}

console.log(`naive fits: ${naiveFits}, exactFits: ${exactFits}`);
