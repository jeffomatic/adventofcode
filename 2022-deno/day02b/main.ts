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

function getResponse(them: Choice, outcome: Outcome): Choice {
  if (outcome == Outcome.Draw) {
    return them;
  }

  if (outcome == Outcome.Win) {
    switch (them) {
      case Choice.Rock:
        return Choice.Paper;
      case Choice.Paper:
        return Choice.Scissors;
      case Choice.Scissors:
        return Choice.Rock;
    }
  }

  // Lose
  switch (them) {
    case Choice.Rock:
      return Choice.Scissors;
    case Choice.Paper:
      return Choice.Rock;
    case Choice.Scissors:
      return Choice.Paper;
  }
}

function parseChoice(s: string): Choice {
  switch (s) {
    case "A":
      return Choice.Rock;
    case "B":
      return Choice.Paper;
    case "C":
      return Choice.Scissors;
    default:
      throw new Error(`invalid input ${s}`);
  }
}

function parseOutcome(s: string): Outcome {
  switch (s) {
    case "X":
      return Outcome.Lose;
    case "Y":
      return Outcome.Draw;
    case "Z":
      return Outcome.Win;
    default:
      throw new Error(`invalid input ${s}`);
  }
}

const res = readInput()
  .split("\n")
  .map((s) => {
    const them = parseChoice(s[0]);
    const outcome = parseOutcome(s[2]);
    const us = getResponse(them, outcome);
    return score(us, outcome);
  })
  .reduce((memo, v) => memo + v);

console.log(res);
