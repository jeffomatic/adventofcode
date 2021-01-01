open Batteries

type mutation_op = Dec | Inc

type condition_op = Eq | Neq | Lt | Lte | Gt | Gte

type mutation = {reg: string; op: mutation_op; param: int}

type condition = {reg: string; op: condition_op; param: int}

type command = {mut: mutation; cond: condition}

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
  | [mut_str; cond_str] ->
      {mut= parse_mutation mut_str; cond= parse_condition cond_str}
  | _ -> failwith ("Invalid command: " ^ str)

let check_val reg state =
  if Map.String.mem reg state then Map.String.find reg state else 0

let mutate (mut: mutation) state =
  let regval = check_val mut.reg state in
  let regval =
    match mut.op with Dec -> regval - mut.param | Inc -> regval + mut.param
  in
  Map.String.add mut.reg regval state

let check_condition (cond: condition) state =
  let regval = check_val cond.reg state in
  match cond.op with
  | Eq -> regval = cond.param
  | Neq -> regval <> cond.param
  | Lt -> regval < cond.param
  | Lte -> regval <= cond.param
  | Gt -> regval > cond.param
  | Gte -> regval >= cond.param

let max_reg state =
  Map.String.fold
    (fun k v acc ->
      let _, old_max = acc in
      if old_max < v then (k, v) else acc )
    state ("__invalid__", min_int)

let run commands =
  List.fold_left
    (fun state cmd ->
      if check_condition cmd.cond state then mutate cmd.mut state else state )
    Map.String.empty commands

let () =
  IO.stdin |> IO.read_all
  |> Str.split (Str.regexp "\n+")
  |> List.map parse_command |> run |> max_reg |> dump |> print_endline
