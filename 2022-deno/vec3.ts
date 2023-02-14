export type Vec3 = [number, number, number];

export function make(x: number, y: number, z: number): Vec3 {
  return [x, y, z];
}

export function copy(src: Vec3, dst: Vec3 | undefined = undefined): Vec3 {
  if (dst == undefined) {
    dst = [0, 0, 0];
  }

  dst[0] = src[0];
  dst[1] = src[1];
  dst[2] = src[2];

  return dst;
}

// Generates a key usable for Record<string, T> or Set<string>
export function key(v: Vec3): string {
  return `${v[0]}.${v[1]}.${v[2]}`;
}

export function manhattan(v: Vec3): number {
  return Math.abs(v[0]) + Math.abs(v[1] + Math.abs(v[2]));
}

export function add(a: Vec3, b: Vec3, out: Vec3 | undefined = undefined): Vec3 {
  if (out == undefined) {
    out = [0, 0, 0];
  }

  out[0] = a[0] + b[0];
  out[1] = a[1] + b[1];
  out[2] = a[2] + b[2];
  return out;
}

export function sub(a: Vec3, b: Vec3, out: Vec3 | undefined = undefined): Vec3 {
  if (out == undefined) {
    out = [0, 0, 0];
  }

  out[0] = a[0] - b[0];
  out[1] = a[1] - b[1];
  out[2] = a[2] - b[2];
  return out;
}

export function equal(a: Vec3, b: Vec3): boolean {
  return a[0] == b[0] && a[1] == b[1] && a[2] == b[2];
}
