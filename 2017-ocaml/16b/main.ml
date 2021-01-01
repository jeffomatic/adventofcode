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

let swap a b arr =
  let tmp = arr.(a) in
  arr.(a) <- arr.(b) ;
  arr.(b) <- tmp ;
  arr

let index_of v arr =
  let rec aux arr v n = if arr.(n) = v then n else aux arr v (n + 1) in
  aux arr v 0

let rec rev a b arr =
  if a >= b then arr else rev (a + 1) (b - 1) (swap a b arr)

let spin n arr =
  let len = Array.length arr in
  rev (len - n) (len - 1) arr |> rev 0 (len - n - 1) |> rev 0 (len - 1)

let partner a b arr = swap (index_of a arr) (index_of b arr) arr

let run_commands commands line =
  List.fold_left
    (fun line c ->
      match c with
      | Spin n -> spin n line
      | Exchange (a, b) -> swap a b line
      | Partner (a, b) -> partner a b line )
    line commands

let string_of_char_arr arr =
  Array.map (fun n -> Char.escaped n) arr |> Array.to_list |> String.join ""

let cycle_size f start =
  let check = Array.copy start in
  let rec aux f prev i = if prev = check then i else aux f (f prev) (i + 1) in
  aux f (f start) 1

let fold_n f start n =
  let rec aux f acc i = if i = n then acc else aux f (f acc) (i + 1) in
  aux f start 0

let read_input () =
  IO.stdin |> IO.read_all |> String.trim |> Str.split (Str.regexp ",")

let () =
  let line = make_line 'p' in
  let input = read_input () in
  let commands = List.map parse_command input in
  let cs = cycle_size (run_commands commands) line in
  let iterations = 1000 * 1000 * 1000 in
  fold_n (run_commands commands) line (iterations mod cs)
  |> string_of_char_arr |> print_endline
