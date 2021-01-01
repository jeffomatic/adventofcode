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

let rec expression_to_string expr reg_aliases =
  match expr with
  | Reg n -> reg_aliases.(n)
  | Imm n -> String.of_int n
  | Operation (op, a, b) -> (
      let a_str = expression_to_string a reg_aliases in
      let b_str = expression_to_string b reg_aliases in
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

let render i ip_reg instruction =
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
  let rval = expression_to_string (reduce rval_expr) reg_aliases in
  let lval =
    if c = ip_reg then "goto" else Printf.sprintf "%s =" reg_aliases.(c)
  in
  Printf.sprintf "%02d: %s %s" i lval rval

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
  in
  (ip_reg, instructions)

let () =
  let ip_reg, instructions = IO.stdin |> IO.read_all |> parse in
  List.mapi (fun i instruction -> render i ip_reg instruction) instructions
  |> String.join "\n" |> print_endline
