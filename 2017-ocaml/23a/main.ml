open Batteries

module Instruction = struct
  type register = int

  type flex_op = Register of register | Immediate of int

  type t =
    | Set of register * flex_op
    | Sub of register * flex_op
    | Mul of register * flex_op
    | Jnz of flex_op * flex_op

  let register_from_string s = int_of_char s.[0] - int_of_char 'a'

  let string_from_register r = Char.escaped (char_of_int (int_of_char 'a' + r))

  let flex_op_from_string s =
    if Str.string_match (Str.regexp "-?[0-9]+") s 0 then
      Immediate (int_of_string s)
    else Register (register_from_string s)

  let string_from_flex_op op =
    match op with
    | Register r -> string_from_register r
    | Immediate v -> string_of_int v

  let parse src =
    match Str.split (Str.regexp " +") src with
    | ["set"; a; b] -> Set (register_from_string a, flex_op_from_string b)
    | ["sub"; a; b] -> Sub (register_from_string a, flex_op_from_string b)
    | ["mul"; a; b] -> Mul (register_from_string a, flex_op_from_string b)
    | ["jnz"; a; b] -> Jnz (flex_op_from_string a, flex_op_from_string b)
    | _ -> failwith ("invalid instruction: " ^ src)
end

module Thread = struct
  type state = Ready | PcOutOfRange

  type t =
    { instructions: Instruction.t Array.t
    ; regs: int Array.t
    ; pc: int
    ; state: state
    ; mul_c: int }

  let make instructions =
    let regs = Array.make (Instruction.register_from_string "h" + 1) 0 in
    {instructions; regs; pc= 0; state= Ready; mul_c= 0}

  let flex_op_val (thread: t) (op: Instruction.flex_op) =
    match op with Register n -> (thread.regs).(n) | Immediate v -> v

  let advance_pc thread n =
    let thread = {thread with pc= thread.pc + n} in
    if thread.pc < Array.length thread.instructions then thread
    else {thread with state= PcOutOfRange}

  let step thread =
    let pca = thread.pc in
    match (thread.instructions).(thread.pc) with
    | Set (a, b) ->
        let va = (thread.regs).(a) in
        let vb = flex_op_val thread b in
        (thread.regs).(a) <- vb ;
        Printf.printf "%d set %s:%d %s:%d\n" pca
          (Instruction.string_from_register a)
          va
          (Instruction.string_from_flex_op b)
          vb ;
        advance_pc thread 1
    | Sub (a, b) ->
        let va = (thread.regs).(a) in
        let vb = flex_op_val thread b in
        (thread.regs).(a) <- va - vb ;
        Printf.printf "%d sub %s:%d %s:%d\n" pca
          (Instruction.string_from_register a)
          va
          (Instruction.string_from_flex_op b)
          vb ;
        advance_pc thread 1
    | Mul (a, b) ->
        let va = (thread.regs).(a) in
        let vb = flex_op_val thread b in
        (thread.regs).(a) <- va * vb ;
        Printf.printf "%d mul %s:%d %s:%d\n" pca
          (Instruction.string_from_register a)
          va
          (Instruction.string_from_flex_op b)
          vb ;
        advance_pc {thread with mul_c= thread.mul_c + 1} 1
    | Jnz (a, b) ->
        let va = flex_op_val thread a in
        let vb = flex_op_val thread b in
        Printf.printf "%d jnz %s:%d %s:%d\n" pca
          (Instruction.string_from_flex_op a)
          va
          (Instruction.string_from_flex_op b)
          vb ;
        let n = if va = 0 then 1 else vb in
        advance_pc thread n
end

let run instructions =
  let rec aux thread =
    let thread = Thread.step thread in
    match thread.state with Ready -> aux thread | PcOutOfRange -> thread
  in
  aux (Thread.make instructions)

let () =
  let src = IO.stdin |> IO.read_all in
  let instructions =
    Str.split (Str.regexp "\n+") src
    |> List.map Instruction.parse |> Array.of_list
  in
  let thread = run instructions in
  dump thread.mul_c |> print_endline
