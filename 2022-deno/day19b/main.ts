import { readInputLines } from "../common.ts";

enum Resource {
  Ore = "ore",
  Clay = "clay",
  Obsidian = "obsidian",
  Geode = "geode",
}

type ResTable = {
  ore: number;
  clay: number;
  obsidian: number;
  geode: number;
};

function makeResTable(
  ore: number,
  clay: number,
  obsidian: number,
  geode: number,
): ResTable {
  return { ore, clay, obsidian, geode };
}

type Blueprint = {
  id: number;
  ore: ResTable;
  clay: ResTable;
  obsidian: ResTable;
  geode: ResTable;
  maxOre: number;
  maxClay: number;
  maxObsidian: number;
};

function parse(s: string): Blueprint {
  const pattern =
    /Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian./;
  const matches = s.match(pattern)!;
  const id = parseInt(matches[1]);
  const ore = makeResTable(parseInt(matches[2]), 0, 0, 0);
  const clay = makeResTable(parseInt(matches[3]), 0, 0, 0);
  const obsidian = makeResTable(
    parseInt(matches[4]),
    parseInt(matches[5]),
    0,
    0,
  );
  const geode = makeResTable(
    parseInt(matches[6]),
    0,
    parseInt(matches[7]),
    0,
  );
  const maxOre = Math.max(ore.ore, clay.ore, obsidian.ore, geode.ore);
  const maxClay = Math.max(ore.clay, clay.clay, obsidian.clay, geode.clay);
  const maxObsidian = Math.max(
    ore.obsidian,
    clay.obsidian,
    obsidian.obsidian,
    geode.obsidian,
  );

  return {
    id,
    ore,
    clay,
    geode,
    obsidian,
    maxOre,
    maxClay,
    maxObsidian,
  };
}

type State = {
  time: number;
  funds: ResTable;
  robots: ResTable;
  nextRobot: Resource | undefined;
};

function makeState(timeRemaining: number): State {
  return {
    time: timeRemaining,
    funds: makeResTable(0, 0, 0, 0),
    robots: makeResTable(1, 0, 0, 0),
    nextRobot: undefined,
  };
}

function copyState(
  s: State,
  nextRobot: Resource | undefined = undefined,
): State {
  return {
    time: s.time,
    funds: { ...s.funds },
    robots: { ...s.robots },
    nextRobot: nextRobot ?? s.nextRobot,
  };
}

const currencies = [Resource.Ore, Resource.Clay, Resource.Obsidian];

function getTurnsToBuildRobot(
  funds: ResTable,
  robots: ResTable,
  cost: ResTable,
): number {
  let res = 0;
  for (const resource of currencies) {
    if (cost[resource] < 1) {
      continue;
    }

    if (funds[resource] >= cost[resource]) {
      continue;
    }

    res = Math.max(
      res,
      Math.ceil(
        (cost[resource] - funds[resource]) / robots[resource],
      ),
    );
  }

  return res + 1; // it takes one turn to actually build the robot
}

function solveDfs(
  state: State,
  bp: Blueprint,
  best: number,
  history: Resource[],
): number {
  let turns = 0;
  let nextRobot: Resource | undefined;

  if (state.nextRobot != undefined) {
    const turnsToRobot = getTurnsToBuildRobot(
      state.funds,
      state.robots,
      bp[state.nextRobot],
    );

    if (turnsToRobot <= state.time) {
      nextRobot = state.nextRobot;
      turns = turnsToRobot;
    } else {
      turns = state.time;
    }
  }

  const spend = nextRobot != undefined
    ? bp[nextRobot]
    : makeResTable(0, 0, 0, 0);

  state.funds.ore += (turns * state.robots.ore) - spend.ore;
  state.funds.clay += (turns * state.robots.clay) - spend.clay;
  state.funds.obsidian += (turns * state.robots.obsidian) - spend.obsidian;
  state.funds.geode += turns * state.robots.geode;

  // The new robot does not mine on its first turn.
  if (nextRobot != undefined) {
    state.robots[nextRobot] += 1;
    history.push(nextRobot);
  }

  state.time -= turns;

  // Assume we produce one geode miner per turn from here on out. What's the
  // most number of geodes we could produce, and would that beat our best yet?
  // If not, we shouldn't continue down this line.
  const maxPossibleGeodes = state.funds.geode + // current funds
    state.time * (state.robots.geode) + // production of current geode miners
    state.time * (state.time - 1) / 2; // production of additional miners
  if (maxPossibleGeodes <= best) {
    return best;
  }

  if (best < state.funds.geode) {
    best = state.funds.geode;
  }

  if (state.time < 1) {
    return best;
  }

  // We can only build one robot per term. This means that the most we can
  // spend of a given resource per turn is equal to the maximum cost of
  // of a single robot. This means that for a given robot type, the most we
  // ever need to build is equal to the maximum per-robot spend for the
  // corresponding resource.

  if (state.robots.obsidian > 0) {
    best = Math.max(
      solveDfs(copyState(state, Resource.Geode), bp, best, history.slice()),
    );
  }

  if (
    state.robots.clay > 0 &&
    state.robots.obsidian < bp.maxObsidian
  ) {
    best = Math.max(
      solveDfs(copyState(state, Resource.Obsidian), bp, best, history.slice()),
    );
  }

  if (state.robots.clay < bp.maxClay) {
    best = Math.max(
      solveDfs(copyState(state, Resource.Clay), bp, best, history.slice()),
    );
  }

  if (state.robots.ore < bp.maxOre) {
    best = Math.max(
      solveDfs(copyState(state, Resource.Ore), bp, best, history.slice()),
    );
  }

  return best;
}

const blueprints = readInputLines(import.meta.url).map(parse);
const duration = 32;

let res = 1;
for (const bp of blueprints) {
  res *= solveDfs(makeState(duration), bp, 0, []);
}

console.log(res);
