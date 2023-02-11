import { print, readInputLines } from "../common.ts";

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
    if (this.curDuration != 0) {
      return;
    }

    if (this.cur.type == InsType.Add) {
      this.xreg += this.cur.val;
    }

    if (this.instructions.length > 0) {
      this.cur = this.instructions.shift()!;
      this.curDuration = duration(this.cur.type);
    }

    return;
  }

  shouldDraw(): boolean {
    const pos = (this.cycle - 1) % 40;
    return pos == this.xreg - 1 || pos == this.xreg || pos == this.xreg + 1;
  }
}

const cpu = new Cpu(readInputLines().map(parseIns));

for (let i = 0; i < 6; i++) {
  for (let j = 0; j < 40; j++) {
    if (cpu.shouldDraw()) {
      print("#");
    } else {
      print(".");
    }

    cpu.step();
  }

  print("\n");
}
