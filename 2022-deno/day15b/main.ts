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
const min = 0;
const max = 4000000;

for (let y = min; y < max; y++) {
  const spans = sensors
    .map((s) => getSpanAtRow(s, y))
    .filter((r) => r != undefined) as [number, number][];
  const sorted = spans.sort((a, b) => a[0] - b[0]);
  const merged = mergeSpans(sorted);
  if (merged.length > 1) {
    console.log((merged[0][1] + 1) * 4000000 + y);
    break;
  }
}
