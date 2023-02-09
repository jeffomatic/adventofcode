export function readInput(): string {
  const maxSize = 1024 * 1024;
  const buf = new Uint8Array(maxSize);

  const nbytes = Deno.stdin.readSync(buf);
  if (nbytes == null) {
    throw new Error("unable to read from stdin");
  }

  return new TextDecoder().decode(buf.slice(0, nbytes)).trim();
}

export function printFull(item: any) {
  const enc = new TextEncoder();
  Deno.stdout.writeSync(enc.encode(JSON.stringify(item) + "\n"));
}
