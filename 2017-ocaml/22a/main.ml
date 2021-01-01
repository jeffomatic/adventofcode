open Batteries

type dir = N | S | W | E

let left d = match d with N -> W | S -> E | W -> S | E -> N

let right d = match d with N -> E | S -> W | W -> N | E -> S

let move d from =
  let i, j = from in
  match d with
  | N -> (i - 1, j)
  | S -> (i + 1, j)
  | W -> (i, j - 1)
  | E -> (i, j + 1)

module Pos = struct
  type t = int * int

  let compare (x0, y0) (x1, y1) =
    match Pervasives.compare x0 x1 with
    | 0 -> Pervasives.compare y0 y1
    | c -> c
end

module PosSet = Set.Make (Pos)

let parse lines =
  lines
  |> List.fold_lefti
       (fun memo i line ->
         line
         |> String.fold_lefti
              (fun memo j c ->
                match c with '#' -> PosSet.add (i, j) memo | _ -> memo )
              memo )
       PosSet.empty

let step pos dir map =
  let dir, map =
    if PosSet.mem pos map then (right dir, PosSet.remove pos map)
    else (left dir, PosSet.add pos map)
  in
  let pos = move dir pos in
  (pos, dir, map)

let run pos dir map iters =
  let rec aux n pos dir map infections =
    match n with
    | 0 -> infections
    | _ ->
        let before = PosSet.cardinal map in
        let pos, dir, map = step pos dir map in
        let infections =
          if PosSet.cardinal map > before then infections + 1 else infections
        in
        aux (n - 1) pos dir map infections
  in
  aux iters pos dir map 0

let () =
  let input = IO.stdin |> IO.read_all in
  (* let input = {|
..#
#..
...
|} in *)
  let lines = input |> Str.split (Str.regexp "\n+") in
  let start_map = lines |> parse in
  let start_pos =
    (List.length lines / 2, (lines |> List.first |> String.length) / 2)
  in
  let res = run start_pos N start_map 10000 in
  res |> dump |> print_endline
