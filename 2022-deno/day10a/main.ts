import { readInputLines } from "../common.ts";

enum InsType {
  Noop,
  Add,
}

type NoopIns = {
  type: InsType.Noop;
};

type AddIns = {
  type: InsType.Add;
  val: number;
};

type Ins = NoopIns | AddIns;

function parseIns(s: string): Ins {
  if (s == "noop") {
    return { type: InsType.Noop };
  }

  const toks = s.split(" ");
  return { type: InsType.Add, val: parseInt(toks[1]) };
}

function duration(type: InsType): number {
  switch (type) {
    case InsType.Noop:
      return 1;
    case InsType.Add:
      return 2;
  }
}

class Cpu {
  cycle = 1;
  xreg = 1;

  private instructions: Ins[];
  private cur: Ins;
  private curDuration: number;

  constructor(instructions: Ins[]) {
    this.instructions = instructions;
    this.cur = this.instructions.shift()!;
    this.curDuration = duration(this.cur.type);
  }

  signalStrength(): number {
    return this.cycle * this.xreg;
  }

  step() {
    this.cycle += 1;
    this.curDuration -= 1;

    if (this.curDuration == 0) {
      if (this.cur.type == InsType.Add) {
        this.xreg += this.cur.val;
      }

      this.cur = this.instructions.shift()!;
      this.curDuration = duration(this.cur.type);
    }
  }
}

const cpu = new Cpu(readInputLines().map(parseIns));
let res = 0;
while (true) {
  cpu.step();
  switch (cpu.cycle) {
    case 20:
    case 60:
    case 100:
    case 140:
    case 180:
    case 220:
      res += cpu.signalStrength();
      break;
  }

  if (cpu.cycle == 220) {
    console.log(res);
    break;
  }
}
