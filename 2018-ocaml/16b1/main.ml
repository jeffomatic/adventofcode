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

type op =
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

let ops =
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

let op_to_string op =
  match op with
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

module OpOrder = struct
  type t = op

  let compare = Pervasives.compare
end

module OpSet = Set.Make (OpOrder)

type instruction = op * int * int * int

let compute regs instruction =
  let output = Array.copy regs in
  let op, a, b, c = instruction in
  ( match op with
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

let possible_ops before after instruction =
  List.fold_left
    (fun ops op ->
      let res =
        compute before (op, instruction.(1), instruction.(2), instruction.(3))
      in
      if regs_equal res after then op :: ops else ops )
    [] ops

let build_opcode_candidates samples =
  let tbl = Hashtbl.create (List.length ops) in
  List.iter
    (fun s ->
      let candidates =
        possible_ops s.before s.after s.instruction |> OpSet.of_list
      in
      let n = (s.instruction).(0) in
      let candidates =
        if Hashtbl.mem tbl n then
          let prev_candidates = Hashtbl.find tbl n in
          OpSet.inter prev_candidates candidates
        else candidates
      in
      Hashtbl.replace tbl n candidates )
    samples ;
  tbl

let print_opcode_candidates tbl =
  Hashtbl.to_list tbl
  |> List.sort (fun a b ->
         let an, _ = a in
         let bn, _ = b in
         compare an bn )
  |> List.map (fun item ->
         let n, candidates = item in
         let candidates =
           OpSet.to_list candidates |> List.map op_to_string
           |> List.sort compare
         in
         Printf.sprintf "%d: %s" n (String.join ", " candidates) )
  |> String.join "\n" |> print_endline

let find_single_candidate_code tbl exclude =
  fold
    (fun accum k ->
      if List.mem k exclude then accum
      else if OpSet.cardinal (Hashtbl.find tbl k) = 1 then k
      else accum )
    (-1) (Hashtbl.keys tbl)

let remove_candidate tbl candidate except =
  Hashtbl.iter
    (fun k v ->
      if k != except then Hashtbl.replace tbl k (OpSet.remove candidate v) )
    tbl

let reduce_opcode_candidates tbl =
  let handled = ref [] in
  let finish = ref false in
  while not !finish do
    let code = find_single_candidate_code tbl !handled in
    if code = -1 then finish := true
    else (
      handled := code :: !handled ;
      remove_candidate tbl (OpSet.choose (Hashtbl.find tbl code)) code )
  done

let actual_data = IO.stdin |> IO.read_all

let () =
  let samples = parse actual_data in
  let opcode_candidates = build_opcode_candidates samples in
  reduce_opcode_candidates opcode_candidates ;
  print_opcode_candidates opcode_candidates
