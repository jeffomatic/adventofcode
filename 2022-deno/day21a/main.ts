import { readInputLines } from "../common.ts";

enum ExprType {
  Value,
  Add,
  Sub,
  Mul,
  Div,
}

type ValueExpr = { type: ExprType.Value; val: number };
type AddExpr = { type: ExprType.Add; a: string; b: string };
type SubExpr = { type: ExprType.Sub; a: string; b: string };
type MulExpr = { type: ExprType.Mul; a: string; b: string };
type DivExpr = { type: ExprType.Div; a: string; b: string };

type Expr = ValueExpr | AddExpr | SubExpr | MulExpr | DivExpr;

function parseExpr(s: string): [string, Expr] {
  const [name, rawExpr] = s.split(": ");

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

function evalExpr(expr: Expr, symbols: Record<string, Expr>): number {
  switch (expr.type) {
    case ExprType.Value:
      return expr.val;
    case ExprType.Add:
      return evalExpr(symbols[expr.a], symbols) +
        evalExpr(symbols[expr.b], symbols);
    case ExprType.Sub:
      return evalExpr(symbols[expr.a], symbols) -
        evalExpr(symbols[expr.b], symbols);
    case ExprType.Mul:
      return evalExpr(symbols[expr.a], symbols) *
        evalExpr(symbols[expr.b], symbols);
    case ExprType.Div:
      return evalExpr(symbols[expr.a], symbols) /
        evalExpr(symbols[expr.b], symbols);
  }
}

const symbols: Record<string, Expr> = {};
for (const line of readInputLines()) {
  const [name, Expr] = parseExpr(line);
  symbols[name] = Expr;
}

const res = evalExpr(symbols["root"], symbols);
console.log(res);
