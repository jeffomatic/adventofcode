import { signWithZero } from "./common.ts";

export type Vec2 = [number, number];

export function make(x: number, y: number): Vec2 {
  return [x, y];
}

export function copy(src: Vec2, dst: Vec2 | undefined = undefined): Vec2 {
  if (dst == undefined) {
    dst = [0, 0];
  }

  dst[0] = src[0];
  dst[1] = src[1];

  return dst;
}

// Generates a key usable for Record<string, T> or Set<string>
export function key(v: Vec2): string {
  return `${v[0]}.${v[1]}`;
}

export function manhattan(v: Vec2): number {
  return Math.abs(v[0]) + Math.abs(v[1]);
}

export function add(a: Vec2, b: Vec2, out: Vec2 | undefined = undefined): Vec2 {
  if (out == undefined) {
    out = [0, 0];
  }

  out[0] = a[0] + b[0];
  out[1] = a[1] + b[1];
  return out;
}

export function sub(a: Vec2, b: Vec2, out: Vec2 | undefined = undefined): Vec2 {
  if (out == undefined) {
    out = [0, 0];
  }

  out[0] = a[0] - b[0];
  out[1] = a[1] - b[1];
  return out;
}

export function equal(a: Vec2, b: Vec2): boolean {
  return a[0] == b[0] && a[1] == b[1];
}

export function* march(from: Vec2, to: Vec2): Generator<Vec2> {
  const delta = sub(to, from);
  if (delta[0] != 0 && delta[1] != 0) {
    throw new Error(`points are diagonal`);
  }

  const step = make(signWithZero(delta[0]), signWithZero(delta[1]));
  let cur = copy(from);
  while (true) {
    yield cur;

    if (equal(cur, to)) {
      break;
    }

    cur = add(cur, step);
  }
}
