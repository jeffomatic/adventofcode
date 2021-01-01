open Batteries

let fetch tbl k default =
  if Hashtbl.mem tbl k then Hashtbl.find tbl k else default

let generate tbl pos =
  let i, j = pos in
  let v =
    fetch tbl (i - 1, j) 0
    + fetch tbl (i + 1, j) 0
    + fetch tbl (i, j - 1) 0
    + fetch tbl (i, j + 1) 0
    + fetch tbl (i - 1, j - 1) 0
    + fetch tbl (i + 1, j - 1) 0
    + fetch tbl (i - 1, j + 1) 0
    + fetch tbl (i + 1, j + 1) 0
  in
  Hashtbl.add tbl (i, j) v ;
  v

let next pos =
  let i, j = pos in
  let layer = max (abs i) (abs j) in
  if j > 0 && -layer < i && i < layer (* right edge *) then (i - 1, j)
  else if i < 0 && j > -layer (* top edge *) then (i, j - 1)
  else if j < 0 && i < layer (* left edge *) then (i + 1, j)
  else (* bottom edge *) (i, j + 1)

let generate_until target =
  let vals = Hashtbl.create 100 in
  Hashtbl.add vals (0, 0) 1 ;
  let rec aux pos =
    let v = generate vals pos in
    if v > target then v else aux (next pos)
  in
  aux (0, 1)

let () =
  let target = 312051 in
  generate_until target |> dump |> print_endline
