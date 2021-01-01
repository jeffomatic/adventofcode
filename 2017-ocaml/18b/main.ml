open Batteries

module Instruction = struct
  type register = int

  type flex_op = Register of register | Immediate of int

  type t =
    | Snd of flex_op
    | Set of register * flex_op
    | Add of register * flex_op
    | Mul of register * flex_op
    | Mod of register * flex_op
    | Rcv of register
    | Jgz of flex_op * flex_op

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
    | ["snd"; a] -> Snd (flex_op_from_string a)
    | ["set"; a; b] -> Set (register_from_string a, flex_op_from_string b)
    | ["add"; a; b] -> Add (register_from_string a, flex_op_from_string b)
    | ["mul"; a; b] -> Mul (register_from_string a, flex_op_from_string b)
    | ["mod"; a; b] -> Mod (register_from_string a, flex_op_from_string b)
    | ["rcv"; a] -> Rcv (register_from_string a)
    | ["jgz"; a; b] -> Jgz (flex_op_from_string a, flex_op_from_string b)
    | _ -> failwith ("invalid construction: " ^ src)
end

module Thread = struct
  type state = Ready | RcvBlock | PcOutOfRange

  type t =
    { id: int
    ; instructions: Instruction.t Array.t
    ; state: state
    ; regs: int Array.t
    ; pc: int
    ; snd_q: int Queue.t
    ; sent: int }

  let make id instructions =
    let regs = Array.make (Instruction.register_from_string "p" + 1) 0 in
    regs.(Instruction.register_from_string "p") <- id ;
    { id
    ; instructions
    ; state= Ready
    ; regs
    ; pc= 0
    ; snd_q= Queue.create ()
    ; sent= 0 }

  let flex_op_val (thread: t) (op: Instruction.flex_op) =
    match op with Register n -> (thread.regs).(n) | Immediate v -> v

  let advance_pc thread n =
    let thread = {thread with pc= thread.pc + n} in
    if thread.pc < Array.length thread.instructions then thread
    else {thread with state= PcOutOfRange}

  let step thread rcv_q =
    let pca = thread.pc in
    match (thread.instructions).(thread.pc) with
    | Snd a ->
        let v = flex_op_val thread a in
        let thread = {thread with sent= thread.sent + 1} in
        Queue.push v thread.snd_q ;
        Printf.printf "[%d] %d snd %s:%d\n" thread.id pca
          (Instruction.string_from_flex_op a)
          v ;
        advance_pc thread 1
    | Set (a, b) ->
        let va = (thread.regs).(a) in
        let vb = flex_op_val thread b in
        (thread.regs).(a) <- vb ;
        Printf.printf "[%d] %d set %s:%d %s:%d\n" thread.id pca
          (Instruction.string_from_register a)
          va
          (Instruction.string_from_flex_op b)
          vb ;
        advance_pc thread 1
    | Add (a, b) ->
        let va = (thread.regs).(a) in
        let vb = flex_op_val thread b in
        (thread.regs).(a) <- va + vb ;
        Printf.printf "[%d] %d add %s:%d %s:%d\n" thread.id pca
          (Instruction.string_from_register a)
          va
          (Instruction.string_from_flex_op b)
          vb ;
        advance_pc thread 1
    | Mul (a, b) ->
        let va = (thread.regs).(a) in
        let vb = flex_op_val thread b in
        (thread.regs).(a) <- va * vb ;
        Printf.printf "[%d] %d mul %s:%d %s:%d\n" thread.id pca
          (Instruction.string_from_register a)
          va
          (Instruction.string_from_flex_op b)
          vb ;
        advance_pc thread 1
    | Mod (a, b) ->
        let va = (thread.regs).(a) in
        let vb = flex_op_val thread b in
        (thread.regs).(a) <- va mod vb ;
        Printf.printf "[%d] %d mod %s:%d %s:%d\n" thread.id pca
          (Instruction.string_from_register a)
          va
          (Instruction.string_from_flex_op b)
          vb ;
        advance_pc thread 1
    | Rcv a ->
        let va = (thread.regs).(a) in
        if Queue.is_empty rcv_q then (
          Printf.printf "[%d] %d rcv %s:%d (blocked)\n" thread.id pca
            (Instruction.string_from_register a)
            va ;
          {thread with state= RcvBlock} )
        else
          let thread = {thread with state= Ready} in
          let vq = Queue.pop rcv_q in
          (thread.regs).(a) <- vq ;
          Printf.printf "[%d] %d rcv %s:%d (%d)\n" thread.id pca
            (Instruction.string_from_register a)
            va vq ;
          advance_pc thread 1
    | Jgz (a, b) ->
        let va = flex_op_val thread a in
        let vb = flex_op_val thread b in
        Printf.printf "[%d] %d jgz %s:%d %s:%d\n" thread.id pca
          (Instruction.string_from_flex_op a)
          va
          (Instruction.string_from_flex_op b)
          vb ;
        let n = if va > 0 then vb else 1 in
        advance_pc thread n
end

let run instructions =
  let rec aux (current_thread: Thread.t) (other_thread: Thread.t) =
    let current_thread = Thread.step current_thread other_thread.snd_q in
    match (current_thread.state, other_thread.state) with
    | Ready, _ -> aux current_thread other_thread
    | _, Ready -> aux other_thread current_thread
    | _, RcvBlock ->
        if Queue.is_empty current_thread.snd_q then
          (current_thread, other_thread)
        else aux other_thread current_thread
    | _, _ -> (current_thread, other_thread)
  in
  aux (Thread.make 0 instructions) (Thread.make 1 instructions)

let () =
  (* let src = "\nsnd 1\nsnd 2\nsnd p\nrcv a\nrcv b\nrcv c\nrcv d\n" in *)
  let src = IO.stdin |> IO.read_all in
  let instructions =
    Str.split (Str.regexp "\n+") src
    |> List.map Instruction.parse |> Array.of_list
  in
  let t1, t2 = run instructions in
  Printf.printf "t%d sent: %d, t%d sent %d\n" t1.id t1.sent t2.id t2.sent
