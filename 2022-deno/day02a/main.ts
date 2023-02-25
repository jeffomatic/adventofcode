import { readInput } from "../common.ts";

enum Outcome {
  Win,
  Lose,
  Draw,
}

enum Choice {
  Rock,
  Paper,
  Scissors,
}

function score(choice: Choice, outcome: Outcome): number {
  let s = 0;
  switch (choice) {
    case Choice.Rock:
      s += 1;
      break;
    case Choice.Paper:
      s += 2;
      break;
    case Choice.Scissors:
      s += 3;
      break;
  }

  switch (outcome) {
    case Outcome.Win:
      s += 6;
      break;
    case Outcome.Draw:
      s += 3;
      break;
    case Outcome.Lose:
      s += 0;
      break;
  }

  return s;
}

function getOutcome(us: Choice, them: Choice): Outcome {
  if (us == them) {
    return Outcome.Draw;
  }

  if (us == Choice.Rock && them == Choice.Scissors) {
    return Outcome.Win;
  }

  if (us == Choice.Scissors && them == Choice.Paper) {
    return Outcome.Win;
  }

  if (us == Choice.Paper && them == Choice.Rock) {
    return Outcome.Win;
  }

  return Outcome.Lose;
}

function parseChoice(s: string): Choice {
  switch (s) {
    case "A":
    case "X":
      return Choice.Rock;
    case "B":
    case "Y":
      return Choice.Paper;
    case "C":
    case "Z":
      return Choice.Scissors;
    default:
      throw new Error(`invalid input ${s}`);
  }
}

const res = readInput()
  .split("\n")
  .map((s) => {
    const them = parseChoice(s[0]);
    const us = parseChoice(s[2]);
    const outcome = getOutcome(us, them);
    return score(us, outcome);
  })
  .reduce((memo, v) => memo + v);

console.log(res);
