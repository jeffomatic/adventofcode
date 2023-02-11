import { readInput } from "../common.ts";

type Node = number | Node[];

enum Ord {
  Lower = -1,
  Higher = 1,
  Equal = 0,
}

function compare(a: Node, b: Node): Ord {
  if (typeof a == "number" && typeof b == "number") {
    if (a < b) {
      return Ord.Lower;
    }

    if (a > b) {
      return Ord.Higher;
    }

    return Ord.Equal;
  }

  if (Array.isArray(a) && Array.isArray(b)) {
    for (let i = 0; i < a.length; i++) {
      // If the right list runs out of items first, the inputs are not in the
      // right order.
      if (b.length <= i) {
        return Ord.Higher;
      }

      switch (compare(a[i], b[i])) {
        case Ord.Lower:
          return Ord.Lower;

        case Ord.Higher:
          return Ord.Higher;

        case Ord.Equal:
          // do nothing;
          break;
      }
    }

    // If the left list runs out of items first, the inputs are in the right
    // order.
    if (a.length < b.length) {
      return Ord.Lower;
    }

    // If the lists are the same length and no comparison makes a decision about
    // the order, continue checking the next part of the input.
    return Ord.Equal;
  }

  // If exactly one value is an integer, convert the integer to a list which
  // contains that integer as its only value, then retry the comparison.
  if (typeof a == "number") {
    // b must be an array
    return compare([a], b);
  }

  // a is an array, b is a number
  return compare(a, [b]);
}

const packets: Node[] = [[2], [6]];
for (const chunk of readInput().split("\n\n")) {
  const [a, b] = chunk.split("\n").map((s) => JSON.parse(s) as Node);
  packets.push(a, b);
}

packets.sort(compare);

const a = packets.findIndex((p) => compare(p, [2]) == Ord.Equal) + 1;
const b = packets.findIndex((p) => compare(p, [6]) == Ord.Equal) + 1;

console.log(a * b);
