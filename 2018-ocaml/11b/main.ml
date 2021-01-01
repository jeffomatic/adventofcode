open Batteries

let power_level x y serial =
  let rack_id = x + 10 in
  (((rack_id * y) + serial) * rack_id mod 1000 / 100) - 5

let make_grid h w serial =
  let grid = Array.make_matrix h w 0 in
  for i = 0 to h - 1 do
    for j = 0 to w - 1 do (grid.(i)).(j) <- power_level (j + 1) (i + 1) serial
    done
  done ;
  grid

let find_submatrix grid =
  let grid_h = Array.length grid in
  let grid_w = Array.length grid.(0) in
  let best_size = ref (-1) in
  let best_power = ref min_int in
  let best_coord = ref (-1, -1) in
  for size = 1 to grid_h do
    for i = 0 to grid_h - size do
      for j = 0 to grid_w - size do
        let tot = ref 0 in
        for yy = i to i + size - 1 do
          for xx = j to j + size - 1 do tot := !tot + (grid.(yy)).(xx) done
        done ;
        if !best_power < !tot then (
          best_size := size ;
          best_power := !tot ;
          best_coord := (i, j) )
      done
    done
  done ;
  let i, j = !best_coord in
  (j + 1, i + 1, !best_size)

let tuple3_to_string t =
  let a, b, c = t in
  Printf.sprintf "(%d, %d, %d)" a b c

let test_find_submatrix () =
  [(18, (90, 269, 16)); (42, (232, 251, 12))]
  |> List.iteri (fun i case ->
         let serial, want = case in
         let got = find_submatrix (make_grid 300 300 serial) in
         if compare got want != 0 then
           failwith
             (Printf.sprintf "case %d: got %s want %s" i (tuple3_to_string got)
                (tuple3_to_string want)) )

(* let () = test_find_submatrix () *)

let () =
  find_submatrix (make_grid 300 300 8979) |> tuple3_to_string |> print_endline
