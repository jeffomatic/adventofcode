open Batteries

type tile = Open | Tree | Lumberyard

let tile_to_string t =
  match t with Open -> "." | Tree -> "|" | Lumberyard -> "#"

let grid_from_string s =
  let lines = s |> String.trim |> String.split_on_char '\n' in
  let grid =
    Array.make_matrix (List.length lines) (String.length (List.hd lines)) Open
  in
  List.iteri
    (fun i line ->
      String.iteri
        (fun j c ->
          (grid.(i)).(j)
          <- ( match c with
             | '.' -> Open
             | '|' -> Tree
             | '#' -> Lumberyard
             | _ -> failwith (Printf.sprintf "invalid tile (%d, %d) %c" i j c)
             ) )
        line )
    lines ;
  grid

let grid_to_string grid =
  grid
  |> Array.map (fun row ->
         row |> Array.map tile_to_string |> Array.to_list |> String.join "" )
  |> Array.to_list |> String.join "\n"

let dimensions grid =
  let h = Array.length grid in
  let w = Array.length grid.(0) in
  (h, w)

let neighbors p dimensions =
  let i, j = p in
  let h, w = dimensions in
  let points =
    [ (i - 1, j - 1)
    ; (i - 1, j)
    ; (i - 1, j + 1)
    ; (i, j - 1)
    ; (i, j + 1)
    ; (i + 1, j - 1)
    ; (i + 1, j)
    ; (i + 1, j + 1) ]
  in
  List.filter
    (fun p ->
      let i, j = p in
      0 <= i && i < h && 0 <= j && j < w )
    points

let tiles grid points =
  List.map
    (fun p ->
      let i, j = p in
      (grid.(i)).(j) )
    points

let count v ls =
  List.fold_left (fun accum item -> if v = item then accum + 1 else accum) 0 ls

let matrix_count v mat =
  Array.fold_left
    (fun accum row ->
      Array.fold_left
        (fun accum tile -> if tile = v then accum + 1 else accum)
        accum row )
    0 mat

let tick grid =
  let h, w = dimensions grid in
  let res = Array.make_matrix h w Open in
  for i = 0 to h - 1 do
    for j = 0 to w - 1 do
      let near_tiles = tiles grid (neighbors (i, j) (h, w)) in
      let num_trees = count Tree near_tiles in
      let num_lumberyards = count Lumberyard near_tiles in
      (res.(i)).(j)
      <- ( match (grid.(i)).(j) with
         | Open -> if num_trees >= 3 then Tree else Open
         | Tree -> if num_lumberyards >= 3 then Lumberyard else Tree
         | Lumberyard ->
             if num_trees >= 1 && num_lumberyards >= 1 then Lumberyard
             else Open )
    done
  done ;
  res

let test_data =
  {|
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
|}

let actual_data = IO.stdin |> IO.read_all

let () =
  let grid = grid_from_string actual_data in
  let grid = fold (fun grid _ -> tick grid) grid (1 -- 10) in
  matrix_count Tree grid * matrix_count Lumberyard grid
  |> dump |> print_endline
