import { readInputLines } from "../common.ts";
import * as vec2 from "../vec2.ts";
type Vec2 = vec2.Vec2;

type Sensor = {
  pos: Vec2;
  beacon: Vec2;
  range: number;
};

type Span = [number, number];

function parseVec(s: string): Vec2 {
  const [xStr, yStr] = s.split(", ");
  const x = parseInt(xStr.slice("x=".length));
  const y = parseInt(yStr.slice("x=".length));
  return vec2.make(x, y);
}

function parseSensor(s: string): Sensor {
  const [posStr, beaconStr] = s.split(": ");
  const pos = parseVec(posStr.slice("Sensor at ".length));
  const beacon = parseVec(beaconStr.slice("closest beacon is at ".length));
  const range = vec2.manhattan(vec2.sub(beacon, pos));
  return { pos, beacon, range };
}

function spanContains(span: Span, val: number): boolean {
  return span[0] <= val && val <= span[1];
}

function getSpanAtRow(s: Sensor, y: number): Span | undefined {
  const yDist = Math.abs(s.pos[1] - y);
  if (yDist > s.range) {
    return undefined;
  }

  const xRange = s.range - yDist;
  return [s.pos[0] - xRange, s.pos[0] + xRange];
}

function shouldMerge(a: Span, b: Span): boolean {
  // Expand b so that we merge spans that are barely touching
  const bExpanded = [b[0] - 1, b[1] + 1];
  return a[0] <= bExpanded[1] && bExpanded[0] <= a[1];
}

function merge(a: Span, b: Span): Span {
  return [Math.min(a[0], b[0]), Math.max(a[1], b[1])];
}

function mergeSpans(spans: Span[]): Span[] {
  if (spans.length < 2) {
    return spans;
  }

  const [first, ...rest] = spans;
  const mergedRest = mergeSpans(rest);
  if (shouldMerge(first, mergedRest[0])) {
    mergedRest[0] = merge(mergedRest[0], first);
    return mergeSpans(mergedRest);
  }

  return [first, ...mergedRest];
}

const sensors = readInputLines().map(parseSensor);
const row = 2000000;
const rowBeacons = new Set(
  sensors.filter((s) => s.beacon[1] == row).map((s) => s.beacon[0]),
);
const spans = sensors
  .map((s) => getSpanAtRow(s, row))
  .filter((r) => r != undefined) as [number, number][];
const sorted = spans.sort((a, b) => a[0] - b[0]);
const merged = mergeSpans(sorted);

let res = 0;
for (const s of merged) {
  res += s[1] - s[0] + 1;
  for (const b of rowBeacons) {
    if (spanContains(s, b)) {
      res -= 1;
    }
  }
}

console.log(res);

/*
// Brute force implementation

let minX = Number.POSITIVE_INFINITY;
let maxX = Number.NEGATIVE_INFINITY;
for (const s of sensors) {
  minX = Math.min(minX, s.pos[0] - s.range);
  maxX = Math.max(maxX, s.pos[0] + s.range);
}

let newRes = 0;
for (let i = minX; i <= maxX; i++) {
  if (rowBeacons.has(i)) {
    continue;
  }

  for (const s of sensors) {
    const dist = vec2.manhattan(vec2.sub(vec2.make(i, row), s.pos));
    if (dist <= s.range) {
      newRes += 1;
      break;
    }
  }
}

console.log(newRes);
*/
