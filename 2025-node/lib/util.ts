import * as fs from 'fs';
import * as path from 'path';

export function readlines(filePath: string): string[] {
  return fs.readFileSync(filePath, 'utf-8').trim().split('\n');
}

function loadFile(filename: string, depth = 1) {
  const e = new Error();
  const line = e.stack!.split('\n')[1 + depth] ?? '';
  const prefix = line.match(/.*\((.*)main\.ts.*/)![1];
  return readlines(path.join(prefix, filename));
}

export function input(): string[] {
  return loadFile('input', 2);
}

export function example(): string[] {
  return loadFile('example', 2);
}
