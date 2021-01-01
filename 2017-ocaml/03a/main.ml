let edge_size target =
  let rec aux target prev_size prev_max =
    let new_size = prev_size + 2 in
    let new_max = prev_max + (4 * prev_size) + 4 in
    if target < new_max then (new_size, prev_max)
    else aux target new_size new_max
  in
  aux target 1 1

let dxdy target =
  let size, prev_max = edge_size target in
  let half_size = size / 2 in
  let ur = prev_max + (size - 1) in
  let ul = ur + (size - 1) in
  let ll = ul + (size - 1) in
  if target <= ur then (half_size, -half_size + (target - prev_max))
  else if target <= ul then (half_size - (target - ur), half_size)
  else if target <= ll then (-half_size, half_size - (target - ul))
  else (-half_size + (target - ll), -half_size)

let () =
  let target = 312051 in
  let dx, dy = dxdy target in
  abs dx + abs dy |> string_of_int |> print_endline
