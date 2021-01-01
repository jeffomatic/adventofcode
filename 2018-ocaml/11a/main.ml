open Batteries

let putv v = dump v |> print_endline

let power_level x y serial =
  let rack_id = x + 10 in
  (((rack_id * y) + serial) * rack_id mod 1000 / 100) - 5

let test_power_level () =
  [(122, 79, 57, -5); (217, 196, 39, 0); (101, 153, 71, 4)]
  |> List.iteri (fun i case ->
         let x, y, serial, want = case in
         let got = power_level x y serial in
         if got != want then
           failwith (Printf.sprintf "case %d: got %d want %d" i got want) )

let make_grid h w serial =
  let grid = Array.make_matrix h w 0 in
  for i = 0 to h - 1 do
    for j = 0 to w - 1 do (grid.(i)).(j) <- power_level (j + 1) (i + 1) serial
    done
  done ;
  grid

let find_submatrix grid h w =
  let grid_h = Array.length grid in
  let grid_w = Array.length grid.(0) in
  let max = ref min_int in
  let best = ref (-1, -1) in
  for i = 0 to grid_h - h do
    for j = 0 to grid_w - w do
      let tot = ref 0 in
      for yy = i to i + h - 1 do
        for xx = j to j + w - 1 do tot := !tot + (grid.(yy)).(xx) done
      done ;
      if !max < !tot then (
        max := !tot ;
        best := (i, j) )
    done
  done ;
  let i, j = !best in
  (j + 1, i + 1)

let tuple2_to_string t =
  let a, b = t in
  Printf.sprintf "(%d, %d)" a b

let test_find_submatrix () =
  [(18, (33, 45)); (42, (21, 61))]
  |> List.iteri (fun i case ->
         let serial, want = case in
         let got = find_submatrix (make_grid 300 300 serial) 3 3 in
         if compare got want != 0 then
           failwith
             (Printf.sprintf "case %d: got %s want %s" i (tuple2_to_string got)
                (tuple2_to_string want)) )

let () =
  find_submatrix (make_grid 300 300 8979) 3 3
  |> tuple2_to_string |> print_endline
