open Batteries

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

let opcode_to_op opcode =
  match opcode with
  | 0 -> Mulr
  | 1 -> Addr
  | 2 -> Banr
  | 3 -> Eqir
  | 4 -> Muli
  | 5 -> Setr
  | 6 -> Eqri
  | 7 -> Gtri
  | 8 -> Eqrr
  | 9 -> Addi
  | 10 -> Gtir
  | 11 -> Gtrr
  | 12 -> Borr
  | 13 -> Bani
  | 14 -> Seti
  | 15 -> Bori
  | _ -> failwith (Printf.sprintf "invalid opcode %d" opcode)

let compute regs instruction =
  let op, a, b, c = instruction in
  match op with
  | Addr -> regs.(c) <- regs.(a) + regs.(b)
  | Addi -> regs.(c) <- regs.(a) + b
  | Mulr -> regs.(c) <- regs.(a) * regs.(b)
  | Muli -> regs.(c) <- regs.(a) * b
  | Banr -> regs.(c) <- regs.(a) land regs.(b)
  | Bani -> regs.(c) <- regs.(a) land b
  | Borr -> regs.(c) <- regs.(a) lor regs.(b)
  | Bori -> regs.(c) <- regs.(a) lor b
  | Setr -> regs.(c) <- regs.(a)
  | Seti -> regs.(c) <- a
  | Gtir -> regs.(c) <- (if a > regs.(b) then 1 else 0)
  | Gtri -> regs.(c) <- (if regs.(a) > b then 1 else 0)
  | Gtrr -> regs.(c) <- (if regs.(a) > regs.(b) then 1 else 0)
  | Eqir -> regs.(c) <- (if a = regs.(b) then 1 else 0)
  | Eqri -> regs.(c) <- (if regs.(a) = b then 1 else 0)
  | Eqrr -> regs.(c) <- (if regs.(a) = regs.(b) then 1 else 0)

let parse data =
  data |> String.trim |> String.split_on_char '\n'
  |> List.map (fun line ->
         Scanf.sscanf line "%d %d %d %d" (fun opcode a b c ->
             (opcode_to_op opcode, a, b, c) ) )

let () =
  let instructions = IO.stdin |> IO.read_all |> parse in
  let regs = [|0; 0; 0; 0|] in
  List.iter (compute regs) instructions ;
  regs.(0) |> dump |> print_endline
