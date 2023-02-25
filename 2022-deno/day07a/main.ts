import { readInput } from "../common.ts";

const lines = readInput().split("\n");
type Execution = {
  cmd: string;
  output: string[];
};

const execs: Execution[] = [];
let i = 0;
while (true) {
  const cmd = lines[i];
  i += 1;

  const output: string[] = [];
  while (i < lines.length && !lines[i].startsWith("$")) {
    output.push(lines[i]);
    i += 1;
  }

  execs.push({ cmd, output });

  if (i >= lines.length) {
    break;
  }
}

const cwd: string[] = [];
const dirSize: Record<string, number> = {};
for (const { cmd, output } of execs) {
  const toks = cmd.split(" ").slice(1);
  switch (toks[0]) {
    case "cd":
      {
        switch (toks[1]) {
          case "..":
            cwd.pop();
            break;
          default:
            cwd.push(toks[1]);
            break;
        }
      }
      break;

    case "ls":
      {
        for (const line of output) {
          if (line.startsWith("dir")) {
            continue;
          }

          const toks = line.split(" ");
          const size = parseInt(toks[0]);
          for (let i = 0; i < cwd.length; i++) {
            const path = cwd.slice(0, i + 1).join("/");
            dirSize[path] ??= 0;
            dirSize[path] += size;
          }
        }
      }
      break;
  }
}

const res = Object.values(dirSize)
  .filter((n) => n <= 100000)
  .reduce((memo, n) => memo + n);

console.log(res);
