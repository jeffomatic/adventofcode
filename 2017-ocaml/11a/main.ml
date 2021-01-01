open Batteries

type dir = N | NE | SE | S | SW | NW

let dir_of_string s =
  match s with
  | "n" -> N
  | "ne" -> NE
  | "se" -> SE
  | "s" -> S
  | "sw" -> SW
  | "nw" -> NW
  | _ -> failwith ("Invalid dir string: " ^ s)

let move from d =
  let x, y = from in
  match d with
  | N -> (x, y - 2)
  | NE -> (x + 1, y - 1)
  | SE -> (x + 1, y + 1)
  | S -> (x, y + 2)
  | SW -> (x - 1, y + 1)
  | NW -> (x - 1, y - 1)

let apply_moves moves = List.fold_left move (0, 0) moves

let distance pos =
  let x, y = pos in
  let x = abs x in
  let y = abs y in
  if x > y then y + (x - y) else x + ((y - x) / 2)

let read_input () =
  IO.stdin |> IO.read_all |> String.trim |> Str.split (Str.regexp ",")

let () =
  let input = read_input () in
  input |> List.map dir_of_string |> apply_moves |> distance |> dump
  |> print_endline
