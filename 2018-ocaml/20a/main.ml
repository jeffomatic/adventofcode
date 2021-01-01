open Batteries

type step = N | S | W | E

and sequence_node = Step of step | Tree of tree

and sequence = sequence_node list

and tree_node = Empty | Sequence of sequence

and tree = tree_node list

let reverse step = match step with N -> S | S -> N | W -> E | E -> W

let pos_to_string p =
  let i, j = p in
  Printf.sprintf "(%d, %d)" i j

let step_to_string s = match s with N -> "N" | S -> "S" | W -> "W" | E -> "E"

let tree_from_string s =
  let chars = s |> String.trim |> String.to_list |> Array.of_list in
  let rec aux n tree seq =
    match chars.(n) with
    | '^' -> aux (n + 1) tree seq
    | 'N' -> aux (n + 1) tree (seq @ [Step N])
    | 'S' -> aux (n + 1) tree (seq @ [Step S])
    | 'W' -> aux (n + 1) tree (seq @ [Step W])
    | 'E' -> aux (n + 1) tree (seq @ [Step E])
    | '(' ->
        let n, child = aux (n + 1) [] [] in
        let seq = seq @ [Tree child] in
        aux (n + 1) tree seq
    | '|' ->
        if List.is_empty seq then
          failwith (Printf.sprintf "invalid | operator at %d" n) ;
        let tree = tree @ [Sequence seq] in
        aux (n + 1) tree []
    | ')' ->
        let tree =
          if List.is_empty seq then tree @ [Empty] else tree @ [Sequence seq]
        in
        (n, tree)
    | '$' -> (n, tree @ [Sequence seq])
    | _ -> failwith (Printf.sprintf "pos %d has invalid char '%c'" n chars.(n))
  in
  let _, tree = aux 0 [] [] in
  tree

let sprint_n s n = fold (fun accum _ -> accum ^ s) "" (1 -- n)

let tree_to_string tree =
  let rec aux tree depth =
    let prefix = sprint_n "\t" depth in
    List.fold_left
      (fun accum node ->
        accum ^ prefix ^ "|\n"
        ^
        match node with
        | Empty -> prefix ^ "\t(EMPTY)\n"
        | Sequence seq ->
            List.fold_left
              (fun accum node ->
                accum
                ^
                match node with
                | Step s -> prefix ^ "\t" ^ step_to_string s ^ "\n"
                | Tree b -> aux b (depth + 1) )
              "" seq )
      "" tree
  in
  aux tree 0

let traverse tree =
  let map = Hashtbl.create (20 * 20) in
  let add_door pos step =
    let doors =
      if Hashtbl.mem map pos then Hashtbl.find map pos else Set.empty
    in
    Hashtbl.replace map pos (Set.add step doors)
  in
  let rec traverse_step source step =
    let i, j = source in
    let outcome =
      match step with
      | N -> (i - 1, j)
      | S -> (i + 1, j)
      | W -> (i, j - 1)
      | E -> (i, j + 1)
    in
    add_door source step ;
    add_door outcome (reverse step) ;
    outcome
  and traverse_sequence source seq =
    List.fold_left
      (fun outcomes seq_node ->
        match seq_node with
        | Step step ->
            Set.fold
              (fun source outcomes ->
                Set.add (traverse_step source step) outcomes )
              outcomes Set.empty
        | Tree tree ->
            Set.fold
              (fun source outcomes ->
                Set.union (traverse_tree source tree) outcomes )
              outcomes Set.empty )
      (Set.add source Set.empty) seq
  and traverse_tree source tree =
    List.fold_left
      (fun outcomes tree_node ->
        match tree_node with
        | Empty -> Set.add source outcomes
        | Sequence seq -> Set.union (traverse_sequence source seq) outcomes )
      Set.empty tree
  in
  ignore (traverse_tree (0, 0) tree) ;
  map

let map_extrema map =
  Hashtbl.fold
    (fun k v accum ->
      let i, j = k in
      let min_i, min_j, max_i, max_j = accum in
      (min i min_i, min j min_j, max i max_i, max j max_j) )
    map
    (max_int, max_int, min_int, min_int)

let map_to_string map =
  let min_i, min_j, max_i, max_j = map_extrema map in
  let h = max_i - min_i + 1 in
  let w = max_j - min_j + 1 in
  Printf.printf "h: %d w: %d\n" h w ;
  let grid = Array.make_matrix ((2 * h) + 1) ((2 * w) + 1) "#" in
  for i = 0 to h - 1 do
    for j = 0 to w - 1 do
      grid.((2 * i) + 1).((2 * j) + 1) <- "."
    done
  done ;
  for i = min_i to max_i do
    for j = min_j to max_j do
      let doors =
        if Hashtbl.mem map (i, j) then Hashtbl.find map (i, j) else Set.empty
      in
      let i = i - min_i in
      let j = j - min_j in
      Set.iter
        (fun door ->
          match door with
          | N -> grid.(2 * i).((2 * j) + 1) <- "-"
          | S -> grid.((2 * i) + 2).((2 * j) + 1) <- "-"
          | W -> grid.((2 * i) + 1).(2 * j) <- "|"
          | E -> grid.((2 * i) + 1).((2 * j) + 2) <- "|" )
        doors
    done
  done ;
  grid.((-2 * min_i) + 1).((-2 * min_j) + 1) <- "X" ;
  grid
  |> Array.map (fun row -> row |> Array.to_list |> String.join "")
  |> Array.to_list |> String.join "\n"

let farthest_room map =
  let rec aux next visited steps =
    let next, visited =
      Set.fold
        (fun pos accum ->
          let next, visited = accum in
          let visited = Set.add pos visited in
          let i, j = pos in
          let doors = Hashtbl.find map pos in
          let next =
            Set.fold
              (fun door next ->
                let p =
                  match door with
                  | N -> (i - 1, j)
                  | S -> (i + 1, j)
                  | W -> (i, j - 1)
                  | E -> (i, j + 1)
                in
                if Set.mem p visited then next else Set.add p next )
              doors next
          in
          (next, visited) )
        next (Set.empty, visited)
    in
    if Set.is_empty next then steps else aux next visited (steps + 1)
  in
  aux (Set.add (0, 0) Set.empty) Set.empty 0

(* 3 doors *)
let test0 = "^WNE$"

(* 10 doors *)
let test1 = "^ENWWW(NEEE|SSE(EE|N))$"

(* 18 doors *)
let test2 = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"

(* 23 doors *)
let test3 = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"

(* 31 doors *)
let test4 = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"

let actual = IO.stdin |> IO.read_all

let data = actual

let () =
  let tree = tree_from_string data in
  (* tree |> tree_to_string |> print_endline ; *)
  let map = traverse tree in
  (* map |> map_to_string |> print_endline ; *)
  Printf.printf "Farthest room: %d steps from origin\n" (farthest_room map)
