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

let op_from_string opstring =
  match opstring with
  | "mulr" -> Mulr
  | "addr" -> Addr
  | "banr" -> Banr
  | "eqir" -> Eqir
  | "muli" -> Muli
  | "setr" -> Setr
  | "eqri" -> Eqri
  | "gtri" -> Gtri
  | "eqrr" -> Eqrr
  | "addi" -> Addi
  | "gtir" -> Gtir
  | "gtrr" -> Gtrr
  | "borr" -> Borr
  | "bani" -> Bani
  | "seti" -> Seti
  | "bori" -> Bori
  | _ -> failwith (Printf.sprintf "invalid opstring %s" opstring)

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

let process ip_reg instructions =
  let regs = [|0; 0; 0; 0; 0; 0|] in
  let rec aux () =
    let ip = regs.(ip_reg) in
    if ip < 0 || Array.length instructions <= ip then regs
    else (
      compute regs instructions.(ip) ;
      regs.(ip_reg) <- regs.(ip_reg) + 1 ;
      aux () )
  in
  aux ()

let parse data =
  let lines = data |> String.trim |> String.split_on_char '\n' in
  let ip_reg_raw = List.hd lines in
  let instructions_raw = List.tl lines in
  let ip_reg = Scanf.sscanf ip_reg_raw "#ip %d" (fun ip_reg -> ip_reg) in
  let instructions =
    instructions_raw
    |> List.map (fun line ->
           Scanf.sscanf line "%s %d %d %d" (fun opstring a b c ->
               (op_from_string opstring, a, b, c) ) )
    |> Array.of_list
  in
  (ip_reg, instructions)

let () =
  let ip_reg, instructions = IO.stdin |> IO.read_all |> parse in
  let regs = process ip_reg instructions in
  regs.(0) |> dump |> print_endline
