open Batteries

type mutation_op = Dec | Inc

type condition_op = Eq | Neq | Lt | Lte | Gt | Gte

type mutation = {reg: string; op: mutation_op; param: int}

type condition = {reg: string; op: condition_op; param: int}

type command = mutation * condition

(* state is a register map plus the running maximum *)
type state = int Map.String.t * int

let parse_mutation str : mutation =
  match Str.split (Str.regexp " ") str with
  | [reg; op_str; param_str] ->
      let op =
        match op_str with
        | "dec" -> Dec
        | "inc" -> Inc
        | _ -> failwith ("Invalid mutation op: " ^ op_str)
      in
      {reg; op; param= int_of_string param_str}
  | _ -> failwith ("Invalid mutation: " ^ str)

let parse_condition str : condition =
  match Str.split (Str.regexp " ") str with
  | [reg; op_str; v_str] ->
      let op =
        match op_str with
        | "==" -> Eq
        | "!=" -> Neq
        | "<" -> Lt
        | "<=" -> Lte
        | ">" -> Gt
        | ">=" -> Gte
        | _ -> failwith ("Invalid condition op: " ^ op_str)
      in
      {reg; op; param= int_of_string v_str}
  | _ -> failwith ("Invalid condition: " ^ str)

let parse_command str =
  match Str.split (Str.regexp " if ") str with
  | [mut_str; cond_str] -> (parse_mutation mut_str, parse_condition cond_str)
  | _ -> failwith ("Invalid command: " ^ str)

let check_val reg registers =
  if Map.String.mem reg registers then Map.String.find reg registers else 0

let mutate (mut: mutation) state =
  let registers, regmax = state in
  let regval = check_val mut.reg registers in
  let regval =
    match mut.op with Dec -> regval - mut.param | Inc -> regval + mut.param
  in
  (Map.String.add mut.reg regval registers, max regmax regval)

let check_condition (cond: condition) registers =
  let regval = check_val cond.reg registers in
  match cond.op with
  | Eq -> regval = cond.param
  | Neq -> regval <> cond.param
  | Lt -> regval < cond.param
  | Lte -> regval <= cond.param
  | Gt -> regval > cond.param
  | Gte -> regval >= cond.param

let run commands =
  List.fold_left
    (fun state cmd ->
      let mut, cond = cmd in
      let registers, _ = state in
      if check_condition cond registers then mutate mut state else state )
    (Map.String.empty, min_int)
    commands

let () =
  let _, regmax =
    IO.stdin |> IO.read_all
    |> Str.split (Str.regexp "\n+")
    |> List.map parse_command |> run
  in
  string_of_int regmax |> print_endline
