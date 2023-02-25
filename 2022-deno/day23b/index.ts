import { iterAll, readInputLines } from "../common.ts";
import * as vec2 from "../vec2.ts";
type Vec2 = vec2.Vec2;

enum Dir {
  NW,
  N,
  NE,
  E,
  SE,
  S,
  SW,
  W,
}

const allDirs = [Dir.NW, Dir.N, Dir.NE, Dir.E, Dir.SE, Dir.S, Dir.SW, Dir.W];

type CardinalDir = Dir.N | Dir.E | Dir.S | Dir.W;

type Proposal = CardinalDir | undefined;

const dirVec: Record<Dir, Vec2> = {
  [Dir.NW]: vec2.make(-1, -1),
  [Dir.N]: vec2.make(-1, 0),
  [Dir.NE]: vec2.make(-1, 1),
  [Dir.E]: vec2.make(0, 1),
  [Dir.SE]: vec2.make(1, 1),
  [Dir.S]: vec2.make(1, 0),
  [Dir.SW]: vec2.make(1, -1),
  [Dir.W]: vec2.make(0, -1),
};

const mustClear: Record<CardinalDir, Vec2[]> = {
  [Dir.N]: [Dir.NW, Dir.N, Dir.NE].map((d) => dirVec[d]),
  [Dir.E]: [Dir.NE, Dir.E, Dir.SE].map((d) => dirVec[d]),
  [Dir.S]: [Dir.SE, Dir.S, Dir.SW].map((d) => dirVec[d]),
  [Dir.W]: [Dir.SW, Dir.W, Dir.NW].map((d) => dirVec[d]),
};

function makeProposalSeq(): CardinalDir[] {
  return [Dir.N, Dir.S, Dir.W, Dir.E];
}

function rotateProposalSeq(seq: CardinalDir[]) {
  seq.push(seq.shift()!);
}

class State {
  positions: Vec2[];
  proposalSeq: CardinalDir[];
  occupancy: Set<string>;

  constructor(positions: Vec2[]) {
    this.positions = positions;
    this.proposalSeq = makeProposalSeq();
    this.occupancy = new Set<string>();

    for (const elf of positions) {
      this.occupancy.add(vec2.key(elf));
    }
  }

  isOccupied(pos: Vec2): boolean {
    return this.occupancy.has(vec2.key(pos));
  }

  isOpen(pos: Vec2): boolean {
    return !this.occupancy.has(vec2.key(pos));
  }

  getProposals(): Proposal[] {
    const proposals: Proposal[] = new Array(this.positions.length);

    for (let i = 0; i < this.positions.length; i++) {
      const pos = this.positions[i];

      // Each Elf considers the eight positions adjacent to themself. If no
      // other Elves are in one of those eight positions, the Elf does not do
      // anything during this round.
      if (iterAll(allDirs, (d) => this.isOpen(vec2.add(pos, dirVec[d])))) {
        continue;
      }

      // Otherwise, the Elf looks in each of four directions in a sequence
      // that shifts by one position each round.
      for (const dir of this.proposalSeq) {
        if (iterAll(mustClear[dir], (v) => this.isOpen(vec2.add(pos, v)))) {
          proposals[i] = dir;
          break;
        }
      }
    }

    return proposals;
  }

  checkProposals(proposals: Proposal[]): boolean[] {
    const hits: Record<string, number> = {};
    const revised: boolean[] = new Array(proposals.length).fill(false);

    for (let i = 0; i < this.positions.length; i++) {
      const proposal = proposals[i];
      if (proposal == undefined) {
        continue;
      }

      const dst = vec2.add(this.positions[i], dirVec[proposal]);
      const key = vec2.key(dst);

      hits[key] ??= 0;
      hits[key] += 1;
    }

    for (let i = 0; i < this.positions.length; i++) {
      const proposal = proposals[i];
      if (proposal == undefined) {
        continue;
      }

      const dst = vec2.add(this.positions[i], dirVec[proposal]);
      const key = vec2.key(dst);
      revised[i] = hits[key] < 2;
    }

    return revised;
  }

  applyProposals(proposals: Proposal[]): boolean {
    const proposalOk = this.checkProposals(proposals);
    let moved = false;

    for (let i = 0; i < proposals.length; i++) {
      const proposal = proposals[i];
      if (proposal == undefined || !proposalOk[i]) {
        continue;
      }

      const prevPos = this.positions[i];
      const nextPos = vec2.add(prevPos, dirVec[proposal]);
      const prevKey = vec2.key(prevPos);
      const nextKey = vec2.key(nextPos);

      this.positions[i] = nextPos;
      this.occupancy.delete(prevKey);
      this.occupancy.add(nextKey);

      moved = true;
    }

    rotateProposalSeq(this.proposalSeq);

    return moved;
  }

  // Returns [[upperLeft-i, upperleft-j], [height, width]]
  bounds(): [Vec2, Vec2] {
    let minX = Number.POSITIVE_INFINITY;
    let minY = Number.POSITIVE_INFINITY;
    let maxX = Number.NEGATIVE_INFINITY;
    let maxY = Number.NEGATIVE_INFINITY;

    for (const [y, x] of this.positions) {
      minX = Math.min(minX, x);
      maxX = Math.max(maxX, x);
      minY = Math.min(minY, y);
      maxY = Math.max(maxY, y);
    }

    return [
      vec2.make(minY, minX),
      vec2.make(maxY - minY + 1, maxX - minX + 1),
    ];
  }

  print(): string {
    const [[minY, minX], [height, width]] = this.bounds();
    let res = "";

    for (let i = 0; i < height; i++) {
      for (let j = 0; j < width; j++) {
        const pos = vec2.make(minY + i, minX + j);
        res += this.isOccupied(pos) ? "#" : ".";
      }
      res += "\n";
    }

    return res;
  }

  static parse(input: string[]): State {
    const height = input.length;
    const width = input[0].length;
    const elves: Vec2[] = [];

    for (let i = 0; i < height; i++) {
      for (let j = 0; j < width; j++) {
        if (input[i][j] == "#") {
          elves.push(vec2.make(i, j));
        }
      }
    }

    return new State(elves);
  }
}

const state = State.parse(readInputLines());

let rounds = 0;
while (true) {
  rounds += 1;
  const proposals = state.getProposals();

  if (!state.applyProposals(proposals)) {
    break;
  }
}

console.log(rounds);
