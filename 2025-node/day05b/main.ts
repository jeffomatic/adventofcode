import { example, input } from '../lib/util';

function maybeMergeRanges(a: [number, number], b: [number, number]): [number, number] | undefined {
  const [aMin, aMax] = a;
  const [bMin, bMax] = b;

  /*
  // A is fully before B
  A|-----| B|-----|

  // A is partially instead B (starting)
  A|-----|
      B|-------|

  // B is fully inside A
  A|-------------|
      B|------|
*/
  if (aMin <= bMax && bMin <= aMax) {
    return [Math.min(aMin, bMin), Math.max(aMax, bMax)];
  }

  return undefined;
}

function sortRanges(ranges: [number, number][]) {
  ranges.sort((a, b) => {
    const [aMin, aMax] = a;
    const [bMin, bMax] = b;

    if (aMin < bMin) {
      return -1;
    }

    if (bMin < aMin) {
      return 1;
    }

    if (aMax < bMax) {
      return -1;
    }

    if (bMax < aMax) {
      return 1;
    }

    return 0;
  });
}

function merge(ranges: [number, number][]): [number, number][] {
  const merged: [number, number][] = [ranges[0]];
  for (let i = 1; i < ranges.length; i++) {
    const cur = merged[merged.length - 1];
    const next = ranges[i];
    const maybeMerged = maybeMergeRanges(cur, next);
    if (maybeMerged != undefined) {
      merged[merged.length - 1] = maybeMerged;
    } else {
      merged.push(next);
    }
  }
  return merged;
}

// const lines = example();
const lines = input();

const blank = lines.indexOf('');
const ranges: [number, number][] = lines
  .slice(0, blank)
  .map((s) => s.split('-').map((c) => parseInt(c)) as [number, number]);
sortRanges(ranges);

function inAnyRange(n: number, ranges: [number, number][]): boolean {
  for (const [min, max] of ranges) {
    if (min <= n && n <= max) {
      return true;
    }
  }

  return false;
}

const merged = merge(ranges);
let count = 0;
for (const [min, max] of merged) {
  count += max - min + 1;
}

console.log(count);
