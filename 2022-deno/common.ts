export function readInput(): string {
  const maxSize = 1024 * 1024;
  const buf = new Uint8Array(maxSize);

  const nbytes = Deno.stdin.readSync(buf);
  if (nbytes == null) {
    throw new Error("unable to read from stdin");
  }

  return new TextDecoder().decode(buf.slice(0, nbytes)).trimEnd();
}

export function readInputLines(): string[] {
  return readInput().split("\n");
}

const textEnc = new TextEncoder();

export function print(s: string) {
  Deno.stdout.writeSync(textEnc.encode(s));
}

export function printFull(item: unknown) {
  Deno.stdout.writeSync(textEnc.encode(JSON.stringify(item) + "\n"));
}

export function isSuperset<T>(set: Set<T>, subset: Set<T>): boolean {
  for (const elem of subset) {
    if (!set.has(elem)) {
      return false;
    }
  }
  return true;
}

export function union<T>(setA: Set<T>, setB: Set<T>): Set<T> {
  const _union = new Set(setA);
  for (const elem of setB) {
    _union.add(elem);
  }
  return _union;
}

export function intersection<T>(setA: Set<T>, setB: Set<T>): Set<T> {
  const _intersection = new Set<T>();
  for (const elem of setB) {
    if (setA.has(elem)) {
      _intersection.add(elem);
    }
  }
  return _intersection;
}

export function symmetricDifference<T>(setA: Set<T>, setB: Set<T>): Set<T> {
  const _difference = new Set(setA);
  for (const elem of setB) {
    if (_difference.has(elem)) {
      _difference.delete(elem);
    } else {
      _difference.add(elem);
    }
  }
  return _difference;
}

export function difference<T>(setA: Set<T>, setB: Set<T>): Set<T> {
  const _difference = new Set(setA);
  for (const elem of setB) {
    _difference.delete(elem);
  }
  return _difference;
}
