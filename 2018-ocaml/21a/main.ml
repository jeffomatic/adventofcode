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

type operator = Add | Multiply | And | Or | GreaterThan | Equals

and operation = operator * expression * expression

and expression = Reg of int | Imm of int | Operation of operation

let registers_to_string regs ip_reg =
  let reg_aliases = [|"a"; "b"; "c"; "d"; "e"; "f"|] in
  reg_aliases.(ip_reg) <- "ip" ;
  regs
  |> Array.mapi (fun i v -> Printf.sprintf "%s: %d" reg_aliases.(i) v)
  |> Array.to_list |> String.join " | "

let rec reduce expr =
  match expr with
  | Operation (op, a, b) -> (
      let a = reduce a in
      let b = reduce b in
      match (a, b) with
      | Imm a, Imm b -> (
        match op with
        | Add -> Imm (a + b)
        | Multiply -> Imm (a * b)
        | _ -> Operation (op, Imm a, Imm b) )
      | _, _ -> Operation (op, a, b) )
  | _ -> expr

let rec expression_to_string expr reg_aliases hex =
  match expr with
  | Reg n -> reg_aliases.(n)
  | Imm n -> if hex then Printf.sprintf "0x%X" n else String.of_int n
  | Operation (op, a, b) -> (
      let hex = match op with And | Or -> true | _ -> false in
      let a_str = expression_to_string a reg_aliases hex in
      let b_str = expression_to_string b reg_aliases hex in
      let a_str =
        match a with Operation _ -> "(" ^ a_str ^ ")" | _ -> a_str
      in
      let b_str =
        match b with Operation _ -> "(" ^ b_str ^ ")" | _ -> b_str
      in
      match op with
      | Add -> a_str ^ " + " ^ b_str
      | Multiply -> a_str ^ " * " ^ b_str
      | And -> a_str ^ " & " ^ b_str
      | Or -> a_str ^ " | " ^ b_str
      | GreaterThan -> a_str ^ " > " ^ b_str
      | Equals -> a_str ^ " == " ^ b_str )

let instruction_to_string i ip_reg instruction =
  let reg_aliases = [|"a"; "b"; "c"; "d"; "e"; "f"|] in
  reg_aliases.(ip_reg) <- "ip" ;
  let op, a, b, c = instruction in
  let reg_rval_expr r = if r = ip_reg then Imm i else Reg r in
  let rval_expr =
    match op with
    | Addr -> Operation (Add, reg_rval_expr a, reg_rval_expr b)
    | Addi -> Operation (Add, reg_rval_expr a, Imm b)
    | Mulr -> Operation (Multiply, reg_rval_expr a, reg_rval_expr b)
    | Muli -> Operation (Multiply, reg_rval_expr a, Imm b)
    | Banr -> Operation (And, reg_rval_expr a, reg_rval_expr b)
    | Bani -> Operation (And, reg_rval_expr a, Imm b)
    | Borr -> Operation (Or, reg_rval_expr a, reg_rval_expr b)
    | Bori -> Operation (Or, reg_rval_expr a, Imm b)
    | Setr -> reg_rval_expr a
    | Seti -> Imm a
    | Gtir -> Operation (GreaterThan, Imm a, reg_rval_expr b)
    | Gtri -> Operation (GreaterThan, reg_rval_expr a, Imm b)
    | Gtrr -> Operation (GreaterThan, reg_rval_expr a, reg_rval_expr b)
    | Eqir -> Operation (Equals, Imm a, reg_rval_expr b)
    | Eqri -> Operation (Equals, reg_rval_expr a, Imm b)
    | Eqrr -> Operation (Equals, reg_rval_expr a, reg_rval_expr b)
  in
  let rval_expr =
    if c = ip_reg then Operation (Add, rval_expr, Imm 1) else rval_expr
  in
  let rval = expression_to_string (reduce rval_expr) reg_aliases false in
  let lval =
    if c = ip_reg then "goto" else Printf.sprintf "%s =" reg_aliases.(c)
  in
  Printf.sprintf "%02d: %s %s" i lval rval

let compute regs instruction i ip_reg =
  let op, a, b, c = instruction in
  ( match op with
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
  | Eqrr -> regs.(c) <- (if regs.(a) = regs.(b) then 1 else 0) ) ;
  Printf.sprintf "%s -> %d (0x%X) %s"
    (instruction_to_string i ip_reg instruction)
    regs.(c) regs.(c)
    (registers_to_string regs ip_reg)
  |> print_endline

let process ip_reg instructions =
  let regs = [|10780777; 0; 0; 0; 0; 0|] in
  let rec aux n =
    let ip = regs.(ip_reg) in
    if ip < 0 || Array.length instructions <= ip then n
    else (
      if ip = 28 then Printf.printf "Reached c == a\n" ;
      compute regs instructions.(ip) ip ip_reg ;
      regs.(ip_reg) <- regs.(ip_reg) + 1 ;
      aux (n + 1) )
  in
  aux 0

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
  process ip_reg instructions |> dump |> print_endline
