open Batteries

type dir = N | S | W | E

let left d = match d with N -> W | S -> E | W -> S | E -> N

let right d = match d with N -> E | S -> W | W -> N | E -> S

let reverse d = match d with N -> S | S -> N | W -> E | E -> W

let move from dir =
  let i, j = from in
  match dir with
  | N -> (i - 1, j)
  | S -> (i + 1, j)
  | W -> (i, j - 1)
  | E -> (i, j + 1)

type state = Clean | Weakened | Infected | Flagged

let check_state map pos =
  if Hashtbl.mem map pos then Hashtbl.find map pos else Clean

let parse lines =
  let map = Hashtbl.create 1000 in
  List.iteri
    (fun i line ->
      String.iteri
        (fun j c -> if c = '#' then Hashtbl.add map (i, j) Infected)
        line )
    lines ;
  map

let step map pos dir =
  let dir, change =
    match check_state map pos with
    | Clean ->
        Hashtbl.replace map pos Weakened ;
        (left dir, Weakened)
    | Weakened ->
        Hashtbl.replace map pos Infected ;
        (dir, Infected)
    | Infected ->
        Hashtbl.replace map pos Flagged ;
        (right dir, Flagged)
    | Flagged ->
        Hashtbl.remove map pos ;
        (reverse dir, Clean)
  in
  let pos = move pos dir in
  (pos, dir, change)

let pos_to_s pos =
  let i, j = pos in
  "(" ^ String.of_int i ^ ", " ^ String.of_int j ^ ")"

let dir_to_s d = match d with N -> "N" | S -> "S" | W -> "W" | E -> "E"

let state_to_s s =
  match s with
  | Clean -> "clean"
  | Weakened -> "weakened"
  | Infected -> "infected"
  | Flagged -> "flagged"

let debug_print map pos dir =
  Printf.printf "%s: %s\n" (pos_to_s pos) (dir_to_s dir) ;
  Hashtbl.iter
    (fun k v -> Printf.printf "\t%s: %s\n" (pos_to_s k) (state_to_s v))
    map

let run pos dir map iters =
  let rec aux n pos dir map infections =
    match n with
    | 0 -> infections
    | _ ->
        let pos, dir, change = step map pos dir in
        (* debug_print map pos dir ; *)
        let infections =
          if change = Infected then infections + 1 else infections
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
  let lines = Str.split (Str.regexp "\n+") input in
  let start_map = parse lines in
  let start_pos =
    (List.length lines / 2, (lines |> List.first |> String.length) / 2)
  in
  let res = run start_pos N start_map 10000000 in
  res |> dump |> print_endline
