import * as fs from 'fs';
import * as path from 'path';

type Opts = { noTrim: true } | undefined;

export function readlines(filePath: string, opts: Opts): string[] {
  let s = fs.readFileSync(filePath, 'utf-8');
  const noTrim = opts?.noTrim ?? false;
  if (!noTrim) {
    s = s.trim();
  }
  return s.split('\n');
}

function loadFile(filename: string, depth = 1, opts: Opts) {
  const e = new Error();
  const line = e.stack!.split('\n')[1 + depth] ?? '';
  const prefix = line.match(/.*\((.*)\/[^/]+\.ts.*/)![1];
  return readlines(path.join(prefix, filename), opts);
}

export function input(opts: Opts = undefined): string[] {
  return loadFile('input', 2, opts);
}

export function example(opts: Opts = undefined): string[] {
  return loadFile('example', 2, opts);
}
