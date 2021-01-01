open Batteries

type team = Elf | Goblin

let team_to_string t = match t with Elf -> "elf" | Goblin -> "goblin"

let pos_equals a b = compare a b = 0

let pos_to_string pos =
  let i, j = pos in
  Printf.sprintf "(%d, %d)" i j

let compare_reading_order a b =
  let ai, aj = a in
  let bi, bj = b in
  if ai < bi then -1
  else if bi < ai then 1
  else if aj < bj then -1
  else if bj < aj then 1
  else 0

type entity = {pos: int * int; team: team; hp: int}

let entity_to_string e =
  let i, j = e.pos in
  Printf.sprintf "(%d, %d) %s %dHP" i j (team_to_string e.team) e.hp

type tile = Empty | Wall | Entity of entity

let parse_map input =
  let lines = input |> String.trim |> String.split_on_char '\n' in
  let h = List.length lines in
  let w = lines |> List.hd |> String.length in
  let map = Array.make_matrix h w Empty in
  lines
  |> List.iteri (fun i line ->
         line
         |> String.iteri (fun j c ->
                (map.(i)).(j)
                <- ( match c with
                   | '.' -> Empty
                   | '#' -> Wall
                   | 'E' -> Entity {pos= (i, j); team= Elf; hp= 200}
                   | 'G' -> Entity {pos= (i, j); team= Goblin; hp= 200}
                   | c ->
                       failwith
                         (Printf.sprintf "(%d, %d): invalid char %c" i j c) )
            ) ) ;
  map

let map_get map pos =
  let i, j = pos in
  (map.(i)).(j)

let map_set map pos v =
  let i, j = pos in
  (map.(i)).(j) <- v

let map_dimensions map = (Array.length map, Array.length map.(0))

let map_to_string map =
  map
  |> Array.map (fun row ->
         row
         |> Array.map (fun tile ->
                match tile with
                | Empty -> "."
                | Wall -> "#"
                | Entity e -> match e.team with Elf -> "E" | Goblin -> "G" )
         |> Array.to_list |> String.join "" )
  |> Array.to_list |> String.join "\n"

let filter_entities map f =
  Array.fold_left
    (fun res row ->
      Array.fold_left
        (fun res tile ->
          match tile with
          | Entity e -> if f e then e :: res else res
          | _ -> res )
        res row )
    [] map
  |> List.sort (fun a b -> compare_reading_order a.pos b.pos)

let entities map = filter_entities map (fun _ -> true)

let enemies_of e map = filter_entities map (fun other -> e.team != other.team)

(* return in read order *)
let adjacents pos =
  let i, j = pos in
  [(i - 1, j); (i, j - 1); (i, j + 1); (i + 1, j)]

let empty_adjacents pos map =
  List.filter (fun pos -> map_get map pos = Empty) (adjacents pos)

let bfs_enemies map e earlyout =
  let h, w = map_dimensions map in
  let distance_map = Array.make_matrix h w max_int in
  let rec aux q enemies dist =
    if List.is_empty q || earlyout enemies then (enemies, distance_map)
    else
      let q, enemies =
        List.fold_left
          (fun accum pos ->
            map_set distance_map pos dist ;
            List.fold_left
              (fun accum pos ->
                let q, enemies = accum in
                match map_get map pos with
                | Entity other ->
                    if e.team != other.team then (q, other :: enemies)
                    else accum
                | Empty -> (pos :: q, enemies)
                | _ -> accum )
              accum (adjacents pos) )
          ([], enemies) q
      in
      let q =
        q
        |> List.sort_uniq compare_reading_order
        |> List.filter (fun pos -> map_get distance_map pos = max_int)
      in
      aux q enemies (dist + 1)
  in
  aux [e.pos] [] 0

let has_adjacent_enemy e map =
  List.exists
    (fun enemy ->
      List.exists (fun pos -> pos_equals enemy.pos pos) (adjacents e.pos) )
    (enemies_of e map)

let move e map =
  (* print_endline "move 1" ; *)
  if has_adjacent_enemy e map then e
  else
    (* print_endline "move 2" ;
    print_endline (entity_to_string e) ; *)
    let nearest_enemies, _ =
      bfs_enemies map e (fun enemies -> not (List.is_empty enemies))
    in
    (* print_endline "move 3" ; *)
    if List.is_empty nearest_enemies then e
    else
      (* print_endline "move 4" ; *)
      let nearest_enemy =
        nearest_enemies
        |> List.sort (fun a b -> compare_reading_order a.pos b.pos)
        |> List.hd
      in
      let _, distance_map =
        bfs_enemies map nearest_enemy (fun entities ->
            entities |> List.exists (fun other -> pos_equals e.pos other.pos)
        )
      in
      let dest =
        empty_adjacents e.pos map
        |> List.fold_left
             (fun accum pos ->
               let dests, min_dist = accum in
               let d = map_get distance_map pos in
               if d < min_dist then ([pos], d)
               else if d = min_dist then (pos :: dests, min_dist)
               else accum )
             ([], max_int)
        |> Tuple2.first
        |> List.sort compare_reading_order
        |> List.hd
      in
      map_set map e.pos Empty ;
      let e = {e with pos= dest} in
      map_set map dest (Entity e) ;
      e

let find_target attacker map =
  (* Printf.printf "%s finding targets...\n" (entity_to_string attacker) ; *)
  adjacents attacker.pos
  |> List.fold_left
       (fun candidates pos ->
         match map_get map pos with
         | Entity e ->
             if e.team != attacker.team then e :: candidates else candidates
         | _ -> candidates )
       []
  |> List.fold_left
       (fun accum e ->
         let candidates, min_hp = accum in
         if e.hp < min_hp then ([e], e.hp)
         else if e.hp = min_hp then (e :: candidates, min_hp)
         else accum )
       ([], max_int)
  |> Tuple2.first
  |> List.sort (fun a b -> compare_reading_order a.pos b.pos)
  |> fun candidates -> List.nth_opt candidates 0

let attack attacker map =
  (* print_endline "move" ; *)
  match find_target attacker map with
  | None -> ()
  | Some target ->
      map_set map target.pos
        ( if target.hp <= 3 then Empty
        else Entity {target with hp= target.hp - 3} )

let tick map =
  List.fold_lefti
    (fun full_round i e ->
      match map_get map e.pos with
      | Empty -> (* entity was killed, do nothing *) full_round
      | _ ->
          let elves =
            filter_entities map (fun e -> e.team = Elf) |> List.length
          in
          let goblins =
            filter_entities map (fun e -> e.team = Goblin) |> List.length
          in
          if elves = 0 || goblins = 0 then false
          else
            let e = move e map in
            attack e map ; true )
    true (entities map)

let test1 =
  {|
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########
|}

let test2 = {|
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
|}

let test3 = {|
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
|}

let test4 = {|
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
|}

let test5 = {|
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
|}

let test6 = {|
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
|}

let test7 =
  {|
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
|}

let actual_data = IO.stdin |> IO.read_all

let () =
  let map = actual_data |> parse_map in
  let rec aux i =
    (* Printf.printf "after round %d\n" i ;
    map |> map_to_string |> print_endline ;
    map |> entities |> List.map entity_to_string |> String.join "\n"
    |> print_endline ;
    print_newline () ; *)
    let full_round = tick map in
    if full_round then aux (i + 1)
    else
      let score =
        i * List.fold_left (fun accum e -> accum + e.hp) 0 (entities map)
      in
      Printf.printf "finished after %d rounds, score %d\n" i score ;
      map_to_string map |> print_endline ;
      entities map |> List.map entity_to_string |> String.join "\n"
      |> print_endline
  in
  aux 0
