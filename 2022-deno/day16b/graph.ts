export type Valve = {
  name: string;
  flow: number;
  neighbors: string[];
};

type Edge = [string, string];

export function parseValve(s: string): Valve {
  const regexp =
    /Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)/;
  const matches = s.match(regexp)!;
  const name = matches[1];
  const flow = parseInt(matches[2]);
  const neighbors = matches[3].split(", ");
  return { name, flow, neighbors };
}

export function prune(valves: Valve[], toRemove: string[]): Valve[] {
  return valves
    .filter((v) => !toRemove.includes(v.name))
    .map((v) => ({
      ...v,
      neighbors: v.neighbors.filter((n) => !toRemove.includes(n)),
    }));
}

export function edgeKey(edge: Edge): string {
  return edge.sort().join(".");
}

function keyToEdges(k: string): Edge {
  const [a, b] = k.split(".");
  return [a, b];
}

export function compactEdgeList(
  valves: Record<string, Valve>,
  start: string,
): Record<string, number> {
  const wantValves = Object.values(valves).filter((v) =>
    v.name == start || v.flow > 0
  );
  const res: Record<string, number> = {};

  for (const v of wantValves) {
    const visited = new Set<string>([v.name]);
    const q: [string, number][] = [[v.name, 0]];

    while (q.length > 0) {
      const [name, dist] = q.shift()!;
      const cur = valves[name];

      if (cur.flow > 0 && dist > 0) {
        res[edgeKey([v.name, cur.name])] = dist;
      }

      for (const n of cur.neighbors) {
        if (visited.has(n)) {
          continue;
        }

        visited.add(n);
        q.push([n, dist + 1]);
      }
    }
  }

  return res;
}

export function dotGraph(valves: Valve[], start: string): string {
  const lines: string[] = [];
  lines.push("graph G {");

  for (const v of valves) {
    if (v.name == start) {
      lines.push(`\t${v.name} [color=purple penwidth=4 root=true]`);
    } else if (v.flow > 0) {
      lines.push(
        `\t${v.name} [color=red penwidth=2 label="${v.name}: ${v.flow}"]`,
      );
    }
  }

  const dedupe = new Set<string>();
  for (const v of valves) {
    for (const n of v.neighbors) {
      const k = [v.name, n].sort().join("");
      if (dedupe.has(k)) {
        continue;
      }
      dedupe.add(k);
      lines.push(`\t${v.name} -- ${n}`);
    }
  }

  lines.push("}");

  return lines.join("\n");
}

export function dotGraphCompact(valves: Valve[], start: string): string {
  const valvesByName = valves.reduce((memo, v) => {
    memo[v.name] = v;
    return memo;
  }, {} as Record<string, Valve>);
  const edgeList = compactEdgeList(valvesByName, start);
  const nodes = new Set<string>(Object.keys(edgeList).flatMap(keyToEdges));

  const lines: string[] = [];
  lines.push("graph G {");

  for (const n of nodes) {
    if (n == "AA") {
      lines.push(`\t${n} [color=purple penwidth=4 root=true]`);
    } else if (valvesByName[n].flow > 0) {
      lines.push(
        `\t${n} [color=red penwidth=2 label="${n}: ${valvesByName[n].flow}"]`,
      );
    }
  }

  for (const k in edgeList) {
    const weight = edgeList[k];
    const [a, b] = keyToEdges(k);
    lines.push(`\t${a} -- ${b} [label="${weight}"]`);
  }

  lines.push("}");

  return lines.join("\n");
}
