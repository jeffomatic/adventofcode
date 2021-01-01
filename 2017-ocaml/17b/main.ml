open Batteries

let item_after_steps step_size stop_after at =
  let rec aux i pos last_at =
    if i > stop_after then last_at
    else
      let pos = 1 + ((pos + step_size) mod i) in
      let last_at = if pos = at then i else last_at in
      aux (i + 1) pos last_at
  in
  aux 1 0 (-1)

let () =
  let step_size = 304 in
  let stop_after = 50 * 1000 * 100 in
  item_after_steps step_size stop_after 1 |> string_of_int |> print_endline
