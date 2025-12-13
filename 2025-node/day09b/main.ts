import { example, input } from '../lib/util';

// A vertical or horizontal span in a 2D space.
type Span = {
  min: number;
  max: number;
  crossAxisPos: number;
};

// const lines = example(); // 24
const lines = input();

const points: [number, number][] = lines.map(
  (line) => line.split(',').map((n) => parseInt(n)) as [number, number],
);

const vertBorders: Span[] = [];
const horzBorders: Span[] = [];

for (let i = 0; i < points.length; i++) {
  const [ax, ay] = points[i];
  const [bx, by] = points[(i + 1) % points.length];

  if (ax == bx) {
    vertBorders.push({
      min: Math.min(ay, by),
      max: Math.max(ay, by),
      crossAxisPos: ax,
    });
  } else {
    horzBorders.push({
      min: Math.min(ax, bx),
      max: Math.max(ax, bx),
      crossAxisPos: ay,
    });
  }
}

// Returns the subset of border spans that contain the given value along the primary axis. This will
// only contain "exterior" borders, as determined by the given cross-axis position and exterior
// direction. The resulting subset is sorted by ascending distance from the cross-axis position; the
// first result is the closest border to the edge on which the given value lies.
function getOverlappingExteriorBorders(
  borders: Span[],
  value: number,
  crossAxisPos: number,
  dir: 1 | -1,
): Span[] {
  return borders
    .filter((border) => {
      if (value < border.min || border.max < value) {
        return false;
      }

      return dir === 1 ? border.crossAxisPos >= crossAxisPos : crossAxisPos >= border.crossAxisPos;
    })
    .sort((a, b) => dir * (a.crossAxisPos - b.crossAxisPos));
}

// Determine if an edge is within the polygon. We can use the set of polygon borders that are
// parallel to the edge.
function checkEdge(edge: Span, borders: Span[], dir: 1 | -1): boolean {
  let current = edge.min;
  while (true) {
    const overlappingSpans = getOverlappingExteriorBorders(
      borders,
      current,
      edge.crossAxisPos,
      dir,
    );

    // An even number of overlapping spans outside of the edge span means the edge sits outside an
    // exterior border of the polygon.
    //
    // This check will false-positive for cases of degenerate rectangles (those that have height or
    // width of 1), but that's fine...none of these degenerate rectangles will produce the answer.
    if (overlappingSpans.length % 2 == 0) {
      return false;
    }

    const nearestSpan = overlappingSpans[0];
    if (nearestSpan.max >= edge.max) {
      return true;
    }

    current = nearestSpan.max + 1;
  }
}

let best = -1;
for (let i = 0; i < points.length - 1; i++) {
  for (let j = i + 1; j < points.length; j++) {
    const [ax, ay] = points[i];
    const [bx, by] = points[j];
    const minX = Math.min(ax, bx);
    const maxX = Math.max(ax, bx);
    const minY = Math.min(ay, by);
    const maxY = Math.max(ay, by);

    // don't bother doing an edge check if the rectangle won't improve our result
    const w = maxX - minX + 1;
    const h = maxY - minY + 1;
    const area = w * h;
    if (area < best) {
      continue;
    }

    // If the polygon has no holes (it can't based on its contruction), and if all four edges of the
    // rectangle are within the polygon, then the whole rectangle is in the polygon.

    // top
    if (!checkEdge({ min: minX, max: maxX, crossAxisPos: minY }, horzBorders, -1)) {
      continue;
    }

    // bottom
    if (!checkEdge({ min: minX, max: maxX, crossAxisPos: maxY }, horzBorders, 1)) {
      continue;
    }

    // left
    if (!checkEdge({ min: minY, max: maxY, crossAxisPos: minX }, vertBorders, -1)) {
      continue;
    }

    // right
    if (!checkEdge({ min: minY, max: maxY, crossAxisPos: maxX }, vertBorders, 1)) {
      continue;
    }

    console.log('new best', points[i], points[j]);
    best = area;
  }
}

console.log(best);
