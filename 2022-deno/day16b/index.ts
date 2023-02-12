import { difference, readInputLines } from "../common.ts";
import { parseValve, prune } from "./graph.ts";
import { search } from "./search.ts";

const valves = readInputLines().map(parseValve);
const maxTime = 26;
const start = "AA";
const allNodes = new Set(valves.map((v) => v.name));
const nodesA = new Set([
  "EP",
  "PZ",
  "DS",
  "DE",
  "MM",
  "UV",
  "YG",
  "MX",
  "JT",
  "WH",
  "AW",
  "RN",
  "LQ",
  "FY",
  "PS",
  "GT",
  "FJ",
  "HW",
  "HF",
  "QN",
  "SN",
  "PY",
]);
const nodesB = difference(allNodes, nodesA);
nodesB.delete(start);

const valvesA = prune(valves, [...nodesB]);
const valvesB = prune(valves, [...nodesA]);

// console.log(dotGraph(valves, start));
// console.log(dotGraph(valvesA, start));
// console.log(dotGraph(valvesB, start));
// console.log(dotGraphCompact(valves, start));
// console.log(dotGraphCompact(valvesA, start));
// console.log(dotGraphCompact(valvesB, start));

console.log(search(valvesA, "AA", maxTime) + search(valvesB, "AA", maxTime));
