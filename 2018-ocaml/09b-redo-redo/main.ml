open Batteries

let puts x = dump x |> print_endline

let insert_marble prevs nexts id after =
  if prevs.(after) = after then (
    prevs.(after) <- id ;
    nexts.(after) <- id ;
    prevs.(id) <- after ;
    nexts.(id) <- after )
  else (
    prevs.(id) <- after ;
    nexts.(id) <- nexts.(after) ;
    prevs.(nexts.(after)) <- id ;
    nexts.(after) <- id )

let remove_marble prevs nexts id =
  let prev = prevs.(id) in
  let next = nexts.(id) in
  prevs.(next) <- prev ; nexts.(prev) <- next

let run nplayers nmarbles =
  let prevs = Array.create (nmarbles + 1) (-1) in
  let nexts = Array.create (nmarbles + 1) (-1) in
  prevs.(0) <- 0 ;
  nexts.(0) <- 0 ;
  let cur = ref 0 in
  let scores = Array.create nplayers 0 in
  for i = 1 to nmarbles do
    if i mod 23 = 0 then (
      let to_remove = fold (fun p _ -> prevs.(p)) !cur (1 -- 7) in
      remove_marble prevs nexts to_remove ;
      cur := nexts.(to_remove) ;
      let player = i mod nplayers in
      scores.(player) <- scores.(player) + i + to_remove )
    else (
      insert_marble prevs nexts i nexts.(!cur) ;
      cur := i )
  done ;
  Array.fold_left max min_int scores

let () = puts (run 452 (100 * 71250))
