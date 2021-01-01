open Batteries

let erosion_modulus = 20183

let mod_a_times_b_plus_c a b c modulus =
  ((a mod modulus * (b mod modulus) mod modulus) + (c mod modulus)) mod modulus

let make_erosion_level_map depth h w =
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
      (map.(y)).(x)
      <- mod_a_times_b_plus_c
           (map.(y)).(x - 1)
           (map.(y - 1)).(x)
           depth erosion_modulus
    done
  done ;
  (* The region at the coordinates of the target has a geologic index of 0. *)
  (map.(h - 1)).(w - 1)
  <- 0 ;
  map

type region_type = Rocky | Wet | Narrow

let region_type_to_string el =
  match el with Rocky -> "." | Wet -> "=" | Narrow -> "|"

let get_region_type el =
  match el mod 3 with
  | 0 -> Rocky
  | 1 -> Wet
  | 2 -> Narrow
  | _ -> failwith "impossible"

let get_region_type_risk el =
  match el with Rocky -> 0 | Wet -> 1 | Narrow -> 2

let make_region_type_map el_map =
  el_map |> Array.map (fun row -> row |> Array.map get_region_type)

let region_type_map_to_string rt_map =
  rt_map
  |> Array.map (fun row ->
         row
         |> Array.map region_type_to_string
         |> Array.to_list |> String.join "" )
  |> Array.to_list |> String.join "\n"

let region_type_map_to_risk_level rt_map =
  Array.fold_left
    (fun accum row ->
      Array.fold_left
        (fun accum rt -> accum + get_region_type_risk rt)
        accum row )
    0 rt_map

type data = {depth: int; target: int * int}

let example_data = {depth= 510; target= (10, 10)}

let actual_data = {depth= 7305; target= (13, 734)}

let () =
  let data = actual_data in
  let el_map =
    make_erosion_level_map data.depth
      (Tuple2.second data.target + 1)
      (Tuple2.first data.target + 1)
  in
  let rt_map = make_region_type_map el_map in
  rt_map |> region_type_map_to_risk_level |> dump |> print_endline
