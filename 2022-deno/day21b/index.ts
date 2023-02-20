import { readInputLines } from "../common.ts";

enum ExprType {
  Value,
  Add,
  Sub,
  Mul,
  Div,
  Match,
  Unknown,
}

type ValueExpr = { type: ExprType.Value; val: number };
type AddExpr = { type: ExprType.Add; a: string; b: string };
type SubExpr = { type: ExprType.Sub; a: string; b: string };
type MulExpr = { type: ExprType.Mul; a: string; b: string };
type DivExpr = { type: ExprType.Div; a: string; b: string };
type MatchExpr = { type: ExprType.Match; a: string; b: string };
type UnknownExpr = { type: ExprType.Unknown };

type Expr =
  | ValueExpr
  | AddExpr
  | SubExpr
  | MulExpr
  | DivExpr
  | MatchExpr
  | UnknownExpr;

function parseExpr(s: string): [string, Expr] {
  const [name, rawExpr] = s.split(": ");

  if (name == "root") {
    const [a, b] = rawExpr.split(" + ");
    return [name, { type: ExprType.Match, a, b }];
  }

  if (name == "humn") {
    return [name, { type: ExprType.Unknown }];
  }

  if (rawExpr.includes("+")) {
    const [a, b] = rawExpr.split(" + ");
    return [name, { type: ExprType.Add, a, b }];
  }

  if (rawExpr.includes("-")) {
    const [a, b] = rawExpr.split(" - ");
    return [name, { type: ExprType.Sub, a, b }];
  }

  if (rawExpr.includes("*")) {
    const [a, b] = rawExpr.split(" * ");
    return [name, { type: ExprType.Mul, a, b }];
  }

  if (rawExpr.includes("/")) {
    const [a, b] = rawExpr.split(" / ");
    return [name, { type: ExprType.Div, a, b }];
  }

  return [name, { type: ExprType.Value, val: parseInt(rawExpr) }];
}

function evalExpr(
  expr: Expr,
  symbols: Record<string, Expr>,
): number | undefined {
  switch (expr.type) {
    case ExprType.Value:
      return expr.val;

    case ExprType.Add: {
      const a = evalExpr(symbols[expr.a], symbols);
      const b = evalExpr(symbols[expr.b], symbols);
      if (a != undefined && b != undefined) {
        return a + b;
      }
      return undefined;
    }

    case ExprType.Sub: {
      const a = evalExpr(symbols[expr.a], symbols);
      const b = evalExpr(symbols[expr.b], symbols);
      if (a != undefined && b != undefined) {
        return a - b;
      }
      return undefined;
    }

    case ExprType.Mul: {
      const a = evalExpr(symbols[expr.a], symbols);
      const b = evalExpr(symbols[expr.b], symbols);
      if (a != undefined && b != undefined) {
        return a * b;
      }
      return undefined;
    }

    case ExprType.Div: {
      const a = evalExpr(symbols[expr.a], symbols);
      const b = evalExpr(symbols[expr.b], symbols);
      if (a != undefined && b != undefined) {
        return a / b;
      }
      return undefined;
    }

    case ExprType.Match:
      return undefined;

    case ExprType.Unknown:
      return undefined;
  }
}

function revEval(
  expr: Expr,
  symbols: Record<string, Expr>,
  want: number,
): number {
  switch (expr.type) {
    case ExprType.Value:
      throw new Error(`attempt to revEval expr ${expr}`);

    case ExprType.Add: {
      const a = evalExpr(symbols[expr.a], symbols);
      if (a != undefined) {
        return revEval(symbols[expr.b], symbols, want - a);
      }

      const b = evalExpr(symbols[expr.b], symbols);
      if (b == undefined) {
        throw new Error(`expected concrete value from ${expr.b}`);
      }

      return revEval(symbols[expr.a], symbols, want - b);
    }

    case ExprType.Sub: {
      const a = evalExpr(symbols[expr.a], symbols);
      if (a != undefined) {
        return revEval(symbols[expr.b], symbols, a - want);
      }

      const b = evalExpr(symbols[expr.b], symbols);
      if (b == undefined) {
        throw new Error(`expected concrete value from ${expr.b}`);
      }

      return revEval(symbols[expr.a], symbols, want + b);
    }

    case ExprType.Mul: {
      const a = evalExpr(symbols[expr.a], symbols);
      if (a != undefined) {
        return revEval(symbols[expr.b], symbols, want / a);
      }

      const b = evalExpr(symbols[expr.b], symbols);
      if (b == undefined) {
        throw new Error(`expected concrete value from ${expr.b}`);
      }

      return revEval(symbols[expr.a], symbols, want / b);
    }

    case ExprType.Div: {
      const a = evalExpr(symbols[expr.a], symbols);
      if (a != undefined) {
        return revEval(symbols[expr.b], symbols, a / want);
      }

      const b = evalExpr(symbols[expr.b], symbols);
      if (b == undefined) {
        throw new Error(`expected concrete value from ${expr.b}`);
      }

      return revEval(symbols[expr.a], symbols, want * b);
    }

    case ExprType.Match: {
      const a = evalExpr(symbols[expr.a], symbols);
      if (a != undefined) {
        return revEval(symbols[expr.b], symbols, a);
      }

      const b = evalExpr(symbols[expr.b], symbols);
      if (b == undefined) {
        throw new Error(`expected concrete value from ${expr.b}`);
      }

      return revEval(symbols[expr.a], symbols, b);
    }

    case ExprType.Unknown:
      return want;
  }
}

const symbols: Record<string, Expr> = {};
for (const line of readInputLines(import.meta.url)) {
  const [name, Expr] = parseExpr(line);
  symbols[name] = Expr;
}

const res = revEval(symbols["root"], symbols, 0);
console.log(res);
