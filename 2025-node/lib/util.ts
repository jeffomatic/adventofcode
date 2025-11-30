import * as fs from 'fs';

export function readlines(filePath: string): string[] {
  return fs.readFileSync(filePath, 'utf-8').trim().split('\n');
}
