import { readInput } from "../common.ts";
import * as vec2 from "../vec2.ts";
type Vec2 = vec2.Vec2;

type Rock = number[][];

const horzBar: Rock = [
  [1, 1, 1, 1],
];
const cross: Rock = [
  [0, 1, 0],
  [1, 1, 1],
  [0, 1, 0],
];
const reverseL: Rock = [
  [0, 0, 1],
  [0, 0, 1],
  [1, 1, 1],
];
const vertBar: Rock = [
  [1],
  [1],
  [1],
  [1],
];
const square: Rock = [
  [1, 1],
  [1, 1],
];

const rockSequence: Rock[] = [horzBar, cross, reverseL, vertBar, square];
const width = 7;

// Positions:
// - x and y axes both start at 0
// - x increases to the left, y decreases upward
// - [0, 0] is the bottom left corner
// - the position of a rock is the position of its upper left corner

class Sim {
  private solid = new Set<number>();
  private minY = 1;
  private width: number;

  constructor(width: number) {
    this.width = width;
  }

  public height(): number {
    return 1 - this.minY;
  }

  public checkCollision(rock: Rock, pos: Vec2): boolean {
    for (let i = 0; i < rock.length; i++) {
      for (let j = 0; j < rock[0].length; j++) {
        if (rock[i][j] == 0) {
          continue;
        }

        const p = vec2.add(pos, [j, i]);
        if (0 < p[1] || p[0] < 0 || this.width <= p[0] || this.isSolid(p)) {
          return true;
        }
      }
    }

    return false;
  }

  public setSolid(rock: Rock, pos: Vec2) {
    for (let i = 0; i < rock.length; i++) {
      for (let j = 0; j < rock[0].length; j++) {
        if (rock[i][j] == 0) {
          continue;
        }

        const p = vec2.add(pos, [j, i]);
        this.solid.add(this.posKey(p));
        this.minY = Math.min(this.minY, p[1]);
      }
    }
  }

  public debugPrint(rock: Rock, rockPos: Vec2) {
    const top = Math.min(rockPos[1], this.minY);

    let s = "";
    for (let i = 0; i < (1 - top); i++) {
      s += "|";

      for (let j = 0; j < width; j++) {
        const p = vec2.make(j, top + i);
        if (this.isSolid(p)) {
          s += "#";
          continue;
        }

        const delta = vec2.sub(p, rockPos);
        if (
          0 <= delta[0] && delta[0] < rock[0].length &&
          0 <= delta[1] && delta[1] < rock.length &&
          rock[delta[1]][delta[0]] != 0
        ) {
          s += "@";
          continue;
        }

        s += ".";
      }

      s += "|\n";
    }

    s += "+";
    for (let i = 0; i < width; i++) {
      s += "-";
    }
    s += "+\n";

    console.log(s);
  }

  public getStartingPos(rock: Rock): Vec2 {
    const x = 2;
    const y = this.minY - 3 - rock.length;
    return [x, y];
  }

  private isSolid(p: Vec2): boolean {
    return this.solid.has(this.posKey(p));
  }

  private posKey(p: Vec2): number {
    return p[1] * this.width + p[0];
  }
}

const debug = false;

const jetSeq = readInput().split("");
const sim = new Sim(7);
const goalRocks = 2022;

let jetIndex = 0;
let rockIndex = 0;
let rockPos = sim.getStartingPos(rockSequence[rockIndex]);
let stoppedRocks = 0;

while (true) {
  const rock = rockSequence[rockIndex];

  if (debug) {
    sim.debugPrint(rockSequence[rockIndex], rockPos);
  }

  // Horizontal motion
  {
    const dir = jetSeq[jetIndex];
    jetIndex = (jetIndex + 1) % jetSeq.length;

    const dx = dir == "<" ? vec2.make(-1, 0) : vec2.make(1, 0);
    const nextPos = vec2.add(rockPos, dx);
    if (!sim.checkCollision(rock, nextPos)) {
      rockPos = nextPos;
    }

    if (debug) {
      console.log(dir);
      sim.debugPrint(rockSequence[rockIndex], rockPos);
    }
  }

  // Downward motion
  {
    const dy = vec2.make(0, 1);
    const nextPos = vec2.add(rockPos, dy);
    if (!sim.checkCollision(rock, nextPos)) {
      if (debug) {
        console.log("v");
      }

      rockPos = nextPos;
    } else {
      if (debug) {
        console.log("next");
      }

      // Freeze the current rock and advance to the next one.
      sim.setSolid(rock, rockPos);
      rockIndex = (rockIndex + 1) % rockSequence.length;
      rockPos = sim.getStartingPos(rockSequence[rockIndex]);

      stoppedRocks += 1;
      if (stoppedRocks == goalRocks) {
        break;
      }
    }
  }
}

console.log(sim.height());
