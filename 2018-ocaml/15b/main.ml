(* This solution is off by one. By pure guesswork, I determined that the goblins
 * seem to get one fewer attack than they should. The elves win at power 10 in
 * 63 rounds, but this simulation produces a total of 1112 remaining HP for the
 * elves, but the correct value is 1109 (score 69867). Gulp. *)

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

type entity = {pos: int * int; team: team; power: int; hp: int}

let entity_to_string e =
  let i, j = e.pos in
  Printf.sprintf "(%d, %d) %s %dHP" i j (team_to_string e.team) e.hp

type tile = Empty | Wall | Entity of entity

let parse_map input elf_power =
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
                   | 'E' ->
                       Entity
                         {pos= (i, j); team= Elf; power= elf_power; hp= 200}
                   | 'G' ->
                       Entity {pos= (i, j); team= Goblin; power= 3; hp= 200}
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

let targets_of e map = filter_entities map (fun other -> e.team != other.team)

(* return in read order *)
let adjacents pos =
  let i, j = pos in
  [(i - 1, j); (i, j - 1); (i, j + 1); (i + 1, j)]

let empty_adjacents pos map =
  List.filter (fun pos -> map_get map pos = Empty) (adjacents pos)

let bfs_targets map e earlyout =
  let h, w = map_dimensions map in
  let distance_map = Array.make_matrix h w max_int in
  let rec aux q targets dist =
    if List.is_empty q || earlyout targets then (targets, distance_map)
    else
      let q, targets =
        List.fold_left
          (fun accum pos ->
            map_set distance_map pos dist ;
            List.fold_left
              (fun accum pos ->
                let q, targets = accum in
                match map_get map pos with
                | Entity other ->
                    if e.team != other.team then (q, other :: targets)
                    else accum
                | Empty -> (pos :: q, targets)
                | _ -> accum )
              accum (adjacents pos) )
          ([], targets) q
      in
      let q =
        q
        |> List.sort_uniq compare_reading_order
        |> List.filter (fun pos -> map_get distance_map pos = max_int)
      in
      aux q targets (dist + 1)
  in
  aux [e.pos] [] 0

let has_adjacent_target e map =
  List.exists
    (fun target ->
      List.exists (fun pos -> pos_equals target.pos pos) (adjacents e.pos) )
    (targets_of e map)

let move e map =
  (* don't move if we're standing next to an target *)
  if has_adjacent_target e map then e
  else
    (* find list of nearest targets, all at the same distance *)
    let nearest_targets, _ =
      bfs_targets map e (fun targets -> not (List.is_empty targets))
    in
    (* all targets may already be dead *)
    if List.is_empty nearest_targets then e
    else
      (* if multiple targets, tie-break using reading order *)
      let nearest_target =
        nearest_targets
        |> List.sort (fun a b -> compare_reading_order a.pos b.pos)
        |> List.hd
      in
      (* get a reverse distance map, i.e. from the target toward the mover *)
      let _, distance_map =
        bfs_targets map nearest_target (fun entities ->
            entities |> List.exists (fun other -> pos_equals e.pos other.pos)
        )
      in
      (* choose a path that the mover will take toward the target, and then take
       * the first step of that path *)
      let dest =
        (* among all of the empty positions adjacent to the mover, find the set
         * of positions that are closest to the target, as indicated by the
         * reverse distance map *)
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

let find_adjacent_targets attacker map =
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
  match find_adjacent_targets attacker map with
  | None -> ()
  | Some target ->
      map_set map target.pos
        ( if target.hp <= attacker.power then Empty
        else Entity {target with hp= target.hp - attacker.power} )

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

let testA =
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

let testB = {|
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
|}

let test1 = {|
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
|}

let test2 = {|
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
|}

let test3 = {|
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
|}

let test4 = {|
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
|}

let test5 =
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
  let rec attempt elf_power =
    let data = actual_data in
    let map = parse_map data elf_power in
    let num_elves =
      entities map |> List.filter (fun e -> e.team = Elf) |> List.length
    in
    let rec aux i =
      (* Printf.printf "after round %d\n" i ;
      map |> map_to_string |> print_endline ;
      map |> entities |> List.map entity_to_string |> String.join "\n"
      |> print_endline ;
      print_newline () ; *)
      let full_round = tick map in
      if full_round then aux (i + 1)
      else
        let survivors = entities map in
        let remaining_elves =
          survivors |> List.filter (fun e -> e.team = Elf) |> List.length
        in
        if num_elves != remaining_elves then attempt (elf_power + 1)
        else
          let total_hp =
            List.fold_left (fun accum e -> accum + e.hp) 0 survivors
          in
          let score = i * total_hp in
          let goblin_damage = (200 * num_elves) - total_hp in
          Printf.printf
            "elf power %d finished after %d rounds, score %d (total hp %d, \
             goblin damage %d)\n"
            elf_power i score total_hp goblin_damage ;
          map_to_string map |> print_endline ;
          survivors |> List.map entity_to_string |> String.join "\n"
          |> print_endline
    in
    aux 0
  in
  attempt 4
