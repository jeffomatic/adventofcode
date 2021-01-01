open Batteries

let erosion_modulus = 20183

let mod_a_times_b_plus_c a b c modulus =
  ((a mod modulus * (b mod modulus) mod modulus) + (c mod modulus)) mod modulus

let make_erosion_level_map depth h w target_pos =
  let tx, ty = target_pos in
  let map = Array.make_matrix h w 0 in
  (* If the region's Y coordinate is 0, the geologic index is its X coordinate times 16807. *)
  for x = 1 to w - 1 do
    (map.(0)).(x) <- mod_a_times_b_plus_c x 16807 depth erosion_modulus
  done ;
  (* If the region's X coordinate is 0, the geologic index is its Y coordinate times 48271. *)
  for y = 1 to h - 1 do
    (map.(y)).(0) <- mod_a_times_b_plus_c y 48271 depth erosion_modulus
  done ;
  (* The region's geologic index is the result of multiplying the
     erosion levels of the regions at X-1,Y and X,Y-1. *)
  for y = 1 to h - 1 do
    for x = 1 to w - 1 do
      (* The region at the coordinates of the target has a geologic index of 0. *)
      if x = tx && y = ty then (map.(y)).(x) <- (0 + depth) mod erosion_modulus
      else (map.(y)).(x)
        <- mod_a_times_b_plus_c
             (map.(y)).(x - 1)
             (map.(y - 1)).(x)
             depth erosion_modulus
    done
  done ;
  let x, y = target_pos in
  (* The region at the coordinates of the target has a geologic index of 0. *)
  (map.(y)).(x) <- 0 ;
  map

type region_type = Rocky | Wet | Narrow

let get_region_type el =
  match el mod 3 with
  | 0 -> Rocky
  | 1 -> Wet
  | 2 -> Narrow
  | _ -> failwith "impossible"

let make_region_type_map el_map target =
  let map =
    el_map |> Array.map (fun row -> row |> Array.map get_region_type)
  in
  map

type tool = ClimbingGear | Torch | Neither

let is_compatible rt tool =
  match (rt, tool) with
  | Rocky, ClimbingGear -> true
  | Rocky, Torch -> true
  | Rocky, Neither -> false
  | Wet, ClimbingGear -> true
  | Wet, Torch -> false
  | Wet, Neither -> true
  | Narrow, ClimbingGear -> false
  | Narrow, Torch -> true
  | Narrow, Neither -> true

let tool_switch_option rt tool =
  match (rt, tool) with
  | Rocky, ClimbingGear -> Torch
  | Rocky, Torch -> ClimbingGear
  | Rocky, Neither -> failwith "invalid rt/tool combination"
  | Wet, ClimbingGear -> Neither
  | Wet, Torch -> failwith "invalid rt/tool combination"
  | Wet, Neither -> ClimbingGear
  | Narrow, ClimbingGear -> failwith "invalid rt/tool combination"
  | Narrow, Torch -> Neither
  | Narrow, Neither -> Torch

let map_get pos map =
  let x, y = pos in
  (map.(y)).(x)

type node = {pos: int * int; tool: tool}

module PQ = struct
  type pg_node = {node: node; rank: int}

  let create min_size =
    let heap =
      Core.Heap.create ~min_size ~cmp:(fun a b -> compare a.rank b.rank) ()
    in
    let add_toks = Hashtbl.create min_size in
    (heap, add_toks)

  let replace pq node rank =
    let heap, add_toks = pq in
    if Hashtbl.mem add_toks node then
      let tok = Hashtbl.find add_toks node in
      let tok = Core.Heap.update heap tok {node; rank} in
      Hashtbl.replace add_toks node tok
    else
      let tok = Core.Heap.add_removable heap {node; rank} in
      Hashtbl.replace add_toks node tok

  let pop pq =
    let heap, add_toks = pq in
    let pqn = Core.Heap.pop_exn heap in
    Hashtbl.remove add_toks pqn.node ;
    (pqn.node, pqn.rank)
end

let get_time node times =
  if Hashtbl.mem node times then Hashtbl.find node times else max_int

let get_best_path rt_map target =
  let min_size = 1000 in
  let visited = Hashtbl.create min_size in
  let times = Hashtbl.create min_size in
  let pq = PQ.create min_size in
  PQ.replace pq {pos= (0, 0); tool= Torch} 0 ;
  let rec aux () =
    let cur_node, cur_time = PQ.pop pq in
    if cur_node = target then cur_time
    else (
      (* mark the current node as visited *)
      Hashtbl.replace visited cur_node true ;
      let actions =
        (* enumerate possible movement options *)
        let x, y = cur_node.pos in
        [(x + 1, y); (x, y + 1); (x - 1, y); (x, y - 1)]
        |> List.filter (fun pos ->
               let x, y = pos in
               x >= 0 && y >= 0 )
        |> List.map (fun pos -> {cur_node with pos})
        |> List.filter (fun action ->
               is_compatible (map_get action.pos rt_map) action.tool )
      in
      (* add tool switch option *)
      let switched_tool =
        tool_switch_option (map_get cur_node.pos rt_map) cur_node.tool
      in
      let actions = actions @ [{cur_node with tool= switched_tool}] in
      let actions =
        actions |> List.filter (fun action -> not (Hashtbl.mem visited action))
      in
      (* add the movement options to the search queue *)
      List.iter
        (fun n ->
          let step_cost = if cur_node.pos = n.pos then 0 else 1 in
          (* Switching to using the climbing gear, torch, or neither always
             takes seven minutes, regardless of which tools you start with. *)
          let step_cost =
            if n.tool = cur_node.tool then step_cost else step_cost + 7
          in
          let time = cur_time + step_cost in
          if time < get_time times n then (
            (* set the new best time for the node *)
            Hashtbl.replace times n time ;
            (* revise the priority queue *)
            PQ.replace pq n time ) )
        actions ;
      aux () )
  in
  aux ()

type data = {depth: int; target: int * int}

let example_data = {depth= 510; target= (10, 10)}

let actual_data = {depth= 7305; target= (13, 734)}

let () =
  let data = actual_data in
  let tx, ty = data.target in
  (* create an extended map in case the route goes past and back to the target *)
  let el_map =
    make_erosion_level_map data.depth (ty * 100) (tx * 100) data.target
  in
  let rt_map = make_region_type_map el_map data.target in
  let target = {pos= data.target; tool= Torch} in
  let time = get_best_path rt_map target in
  Printf.printf "Best path: %d minutes\n" time
