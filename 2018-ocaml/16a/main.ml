open Batteries

let group_by_n n items =
  let groups, rem =
    List.fold_left
      (fun accum i ->
        let groups, rem = accum in
        let rem = i :: rem in
        if List.length rem = n then (List.rev rem :: groups, [])
        else (groups, rem) )
      ([], []) items
  in
  let groups = if List.is_empty rem then groups else List.rev rem :: groups in
  List.rev groups

type sample =
  {before: int Array.t; after: int Array.t; instruction: int Array.t}

let parse data =
  let lines = data |> String.trim |> String.split_on_char '\n' in
  List.map
    (fun raw ->
      let before =
        Scanf.sscanf (List.nth raw 0) "Before: [%d, %d, %d, %d]"
          (fun a b c d -> [|a; b; c; d|] )
      in
      let after =
        Scanf.sscanf (List.nth raw 2) "After: [%d, %d, %d, %d]" (fun a b c d ->
            [|a; b; c; d|] )
      in
      let instruction =
        Scanf.sscanf (List.nth raw 1) "%d %d %d %d" (fun a b c d ->
            [|a; b; c; d|] )
      in
      {before; after; instruction} )
    (group_by_n 4 lines)

type opcode =
  | Addr
  | Addi
  | Mulr
  | Muli
  | Banr
  | Bani
  | Borr
  | Bori
  | Setr
  | Seti
  | Gtir
  | Gtri
  | Gtrr
  | Eqir
  | Eqri
  | Eqrr

let opcodes =
  [ Addr
  ; Addi
  ; Mulr
  ; Muli
  ; Banr
  ; Bani
  ; Borr
  ; Bori
  ; Setr
  ; Seti
  ; Gtir
  ; Gtri
  ; Gtrr
  ; Eqir
  ; Eqri
  ; Eqrr ]

let opcode_to_string opcode =
  match opcode with
  | Addr -> "Addr"
  | Addi -> "Addi"
  | Mulr -> "Mulr"
  | Muli -> "Muli"
  | Banr -> "Banr"
  | Bani -> "Bani"
  | Borr -> "Borr"
  | Bori -> "Bori"
  | Setr -> "Setr"
  | Seti -> "Seti"
  | Gtir -> "Gtir"
  | Gtri -> "Gtri"
  | Gtrr -> "Gtrr"
  | Eqir -> "Eqir"
  | Eqri -> "Eqri"
  | Eqrr -> "Eqrr"

type instruction = opcode * int * int * int

let compute regs instruction =
  let output = Array.copy regs in
  let opcode, a, b, c = instruction in
  ( match opcode with
  | Addr -> output.(c) <- regs.(a) + regs.(b)
  | Addi -> output.(c) <- regs.(a) + b
  | Mulr -> output.(c) <- regs.(a) * regs.(b)
  | Muli -> output.(c) <- regs.(a) * b
  | Banr -> output.(c) <- regs.(a) land regs.(b)
  | Bani -> output.(c) <- regs.(a) land b
  | Borr -> output.(c) <- regs.(a) lor regs.(b)
  | Bori -> output.(c) <- regs.(a) lor b
  | Setr -> output.(c) <- regs.(a)
  | Seti -> output.(c) <- a
  | Gtir -> output.(c) <- (if a > regs.(b) then 1 else 0)
  | Gtri -> output.(c) <- (if regs.(a) > b then 1 else 0)
  | Gtrr -> output.(c) <- (if regs.(a) > regs.(b) then 1 else 0)
  | Eqir -> output.(c) <- (if a = regs.(b) then 1 else 0)
  | Eqri -> output.(c) <- (if regs.(a) = b then 1 else 0)
  | Eqrr -> output.(c) <- (if regs.(a) = regs.(b) then 1 else 0) ) ;
  output

let regs_equal a b =
  a.(0) = b.(0) && a.(1) = b.(1) && a.(2) = b.(2) && a.(3) = b.(3)

let possible_opcodes before after instruction =
  List.fold_left
    (fun opcodes opcode ->
      let res =
        compute before
          (opcode, instruction.(1), instruction.(2), instruction.(3))
      in
      if regs_equal res after then opcode :: opcodes else opcodes )
    [] opcodes

let test_data = {|
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
|}

let actual_data = IO.stdin |> IO.read_all

let () =
  let samples = parse actual_data in
  samples
  |> List.fold_left
       (fun accum s ->
         if List.length (possible_opcodes s.before s.after s.instruction) >= 3
         then accum + 1
         else accum )
       0
  |> dump |> print_endline
