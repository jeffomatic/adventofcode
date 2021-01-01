open Batteries

type operator = Set | Add | Mul | Mod | Snd | Rcv | Jgz

type operand = Register of string | Immediate of int | None

type command = operator * string * operand

type state = {regs: int Map.String.t; pc: int; last_snd: int; last_rcv: int}

let parse_operator str =
  match str with
  | "set" -> Set
  | "add" -> Add
  | "mul" -> Mul
  | "mod" -> Mod
  | "snd" -> Snd
  | "rcv" -> Rcv
  | "jgz" -> Jgz
  | _ -> failwith "invalid operator"

let parse_operand str =
  if Str.string_match (Str.regexp "-?[0-9]+") str 0 then
    Immediate (int_of_string str)
  else Register str

let parse_command str =
  match Str.split (Str.regexp " +") str with
  | [a; b] -> (
    match parse_operator a with
    | Snd -> (Snd, b, None)
    | Rcv -> (Rcv, b, None)
    | _ -> failwith ("invalid command: " ^ str) )
  | [a; b; c] -> (parse_operator a, b, parse_operand c)
  | _ -> failwith ("invalid command: " ^ str)

let regval r state = try Map.String.find r state.regs with Not_found -> 0

let opval op state =
  match op with
  | Register r -> regval r state
  | Immediate n -> n
  | None -> failwith "cannot take value of None"

let setreg r v state = {state with regs= Map.String.add r v state.regs}

let run_command c state =
  let operand, op1, op2 = c in
  match operand with
  | Set -> {(setreg op1 (opval op2 state) state) with pc= state.pc + 1}
  | Add ->
      let v = regval op1 state + opval op2 state in
      {(setreg op1 v state) with pc= state.pc + 1}
  | Mul ->
      let v = regval op1 state * opval op2 state in
      {(setreg op1 v state) with pc= state.pc + 1}
  | Mod ->
      let v = regval op1 state mod opval op2 state in
      {(setreg op1 v state) with pc= state.pc + 1}
  | Snd -> {state with pc= state.pc + 1; last_snd= regval op1 state}
  | Rcv ->
      let last_rcv =
        if state.last_snd <> 0 then state.last_snd else state.last_rcv
      in
      {state with pc= state.pc + 1; last_rcv}
  | Jgz ->
      let delta = if regval op1 state > 0 then opval op2 state else 1 in
      {state with pc= state.pc + delta}

let run commands =
  let rec aux state =
    let state = run_command commands.(state.pc) state in
    match state.last_rcv with 0 -> aux state | n -> n
  in
  aux {regs= Map.String.empty; pc= 0; last_snd= 0; last_rcv= 0}

let read_input () = IO.stdin |> IO.read_all |> Str.split (Str.regexp "\n+")

let () =
  let input = read_input () in
  let commands = List.map parse_command input |> Array.of_list in
  run commands |> string_of_int |> print_endline
