open Batteries

type command = Spin of int | Exchange of int * int | Partner of char * char

let parse_command s =
  if Str.string_match (Str.regexp "^s[0-9]+$") s 0 then
    Scanf.sscanf s "s%d" (fun n -> Spin n)
  else if Str.string_match (Str.regexp "^x[0-9]+/[0-9]+$") s 0 then
    Scanf.sscanf s "x%d/%d" (fun a b -> Exchange (a, b))
  else if Str.string_match (Str.regexp "^p[a-z]/[a-z]$") s 0 then
    Scanf.sscanf s "p%c/%c" (fun a b -> Partner (a, b))
  else failwith ("invalid command: " ^ s)

let make_line last_letter =
  let size = Char.code last_letter - Char.code 'a' + 1 in
  Array.mapi (fun i _ -> Char.code 'a' + i |> Char.chr) (Array.make size 0)

let spin src n =
  let len = Array.length src in
  let dst = Array.make len 'a' in
  for i = 0 to n - 1 do dst.(i) <- src.(i + len - n) done ;
  for i = n to len - 1 do dst.(i) <- src.(i - n) done ;
  dst

let exchange src a b =
  let dst = Array.copy src in
  let tmp = dst.(a) in
  dst.(a) <- dst.(b) ;
  dst.(b) <- tmp ;
  dst

let index_of arr v =
  let rec aux arr v n = if arr.(n) = v then n else aux arr v (n + 1) in
  aux arr v 0

let partner src a b =
  let a = index_of src a in
  let b = index_of src b in
  exchange src a b

let run_commands commands line =
  List.fold_left
    (fun line c ->
      match c with
      | Spin n -> spin line n
      | Exchange (a, b) -> exchange line a b
      | Partner (a, b) -> partner line a b )
    line commands

let read_input () =
  IO.stdin |> IO.read_all |> String.trim |> Str.split (Str.regexp ",")

let () =
  (* let line = make_line 'e' in
  let input = ["s1"; "x3/4"; "pe/b"] in *)
  let line = make_line 'p' in
  let input = read_input () in
  let commands = List.map parse_command input in
  run_commands commands line
  |> Array.map (fun n -> Char.escaped n)
  |> Array.to_list |> String.join "" |> print_endline
