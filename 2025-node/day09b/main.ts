import { example, input } from '../lib/util';

// A vertical or horizontal span in a 2D space.
type PolygonSpan = {
  min: number;
  max: number;
  crossAxisPos: number;

  // Horizontal spans: -1 -> interior above, 1 -> interior below
  // Vertical spans: -1 -> interior left, 1 -> interior right
  interiorDir: number;
};

type RectEdge = {
  min: number;
  max: number;
  crossAxisPos: number;
};

enum Winding {
  CW,
  CCW,
}

// Return the winding of the polygon, along with a starting point.
function checkWinding(points: [number, number][]): Winding {
  let minSpanY = Infinity;
  let minSpanStart = -1;

  // check the directionality of the topmost horizontal span
  for (let i = 0; i < points.length; i++) {
    const [, ay] = points[i];
    const [, by] = points[(i + 1) % points.length];
    if (ay != by) {
      continue;
    }

    if (ay < minSpanY) {
      minSpanStart = i;
      minSpanY = ay;
    }
  }

  const [ax] = points[minSpanStart];
  const [bx] = points[(minSpanStart + 1) % points.length];
  return ax < bx ? Winding.CW : Winding.CCW;
}

function interiorDir(disp: [number, number], winding: Winding): -1 | 1 {
  const [dx, dy] = disp;

  if (dx < 0) {
    return winding == Winding.CW ? -1 : 1;
  }

  if (dx > 0) {
    return winding == Winding.CW ? 1 : -1;
  }

  if (dy < 0) {
    return winding == Winding.CW ? 1 : -1;
  }

  if (dy > 0) {
    return winding == Winding.CW ? -1 : 1;
  }

  throw new Error('zero displacement');
}

function overlap(a: [number, number], b: [number, number]): boolean {
  return a[0] <= b[1] && b[0] <= a[1];
}

function flip(dir: 1 | -1): 1 | -1 {
  return dir == 1 ? -1 : 1;
}

// Returns the subset of border spans that contain the given value along the primary axis. This will
// only contain "exterior" borders, as determined by the given cross-axis position and exterior
// direction. The resulting subset is sorted by ascending distance from the cross-axis position; the
// first result is the closest border to the edge on which the given value lies.
function getOverlappingExteriorBorders(
  borders: PolygonSpan[],
  edge: RectEdge,
  dir: 1 | -1,
): PolygonSpan[] {
  return borders.filter((border) => {
    if (!overlap([edge.min, edge.max], [border.min, border.max])) {
      return false;
    }

    return dir === 1
      ? border.crossAxisPos >= edge.crossAxisPos
      : edge.crossAxisPos >= border.crossAxisPos;
  });
}

// Split all provided spans so that overlapping subspans are all the same size
function splitSpans(spans: PolygonSpan[]): PolygonSpan[] {
  const splitPoints = new Set<number>();
  for (const span of spans) {
    splitPoints.add(span.min);
    splitPoints.add(span.max);
  }
  const sorted = [...splitPoints].sort((a, b) => a - b);
  return spans.flatMap((span) => {
    const subspans: PolygonSpan[] = [];

    // split the span into multiple spans along the split points
    let start = span.min;
    while (true) {
      const end = sorted.find((p) => start < p && p <= span.max);
      if (end == undefined) {
        break;
      }

      subspans.push({ ...span, min: start, max: end });
      start = end;
    }

    if (subspans.length === 0) {
      subspans.push({ ...span });
    }

    return subspans;
  });
}

// Assumes `borders` has been processed by `splitSpans()` and sorted by nearest cross-axis position,
// making it easy for us to discover the nearest border span to the provided point.
function checkEdgePoint(
  mainAxisPos: number,
  crossAxisPos: number,
  borders: PolygonSpan[],
  wantDir: 1 | -1,
): number | undefined {
  const span = borders.find((border) => border.min <= mainAxisPos && mainAxisPos <= border.max);
  if (span === undefined) {
    return undefined;
  }

  // Special case: the point we're checking intersects the border, which is always on "interior"
  // regardless of the border's interior direction
  if (crossAxisPos === span.crossAxisPos) {
    return span.max;
  }

  return span.interiorDir == wantDir ? span.max : undefined;
}

// Determine if an edge is within the polygon. We can use the set of polygon borders that are
// parallel to the edge.
function checkEdge(edge: RectEdge, borders: PolygonSpan[], dir: 1 | -1): boolean {
  // Optimization: we only need to check one point on the nearest span for each chunk of the edge.
  // The exterior borders relative to the provided edge may overlap arbitrarily, so split them into
  // subspans so all overlapping subspans are the same size. This lets us easily find the closest
  // border subspan to the exterior of any point on the edge.
  const overlappingSpans = splitSpans(getOverlappingExteriorBorders(borders, edge, dir)).sort(
    (a, b) => dir * (a.crossAxisPos - b.crossAxisPos),
  );

  let i = edge.min;
  while (i <= edge.max) {
    const next = checkEdgePoint(i, edge.crossAxisPos, overlappingSpans, flip(dir));
    if (!next) {
      return false;
    }

    i = next + 1;
  }

  return true;
}

// const lines = example(); // 24
const lines = input(); // 1568849600

const points: [number, number][] = lines.map(
  (line) => line.split(',').map((n) => parseInt(n)) as [number, number],
);

const vertBorders: PolygonSpan[] = [];
const horzBorders: PolygonSpan[] = [];
const winding = checkWinding(points);

for (let i = 0; i < points.length; i++) {
  const [ax, ay] = points[i];
  const [bx, by] = points[(i + 1) % points.length];
  const disp: [number, number] = [bx - ax, by - ay];

  if (ax == bx) {
    vertBorders.push({
      min: Math.min(ay, by),
      max: Math.max(ay, by),
      crossAxisPos: ax,
      interiorDir: interiorDir(disp, winding),
    });
  } else {
    horzBorders.push({
      min: Math.min(ax, bx),
      max: Math.max(ax, bx),
      crossAxisPos: ay,
      interiorDir: interiorDir(disp, winding),
    });
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

    best = area;
  }
}

console.log(best);
