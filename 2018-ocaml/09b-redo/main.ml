open Batteries

let puts x = dump x |> print_endline

type node = {id: int; prev: int; next: int}

let insert_marble ring id after =
  let after_node = ring.(after) in
  if after_node.prev = after then (
    ring.(id) <- {id; prev= after; next= after} ;
    ring.(after) <- {after_node with prev= id; next= id} )
  else (
    ring.(after) <- {after_node with next= id} ;
    ring.(id) <- {id; prev= after; next= after_node.next} ;
    ring.(after_node.next) <- {(ring.(after_node.next)) with prev= id} )

let remove_marble ring id =
  let node = ring.(id) in
  ring.(node.prev) <- {(ring.(node.prev)) with next= node.next} ;
  ring.(node.next) <- {(ring.(node.next)) with prev= node.prev} ;
  ()

let get_cw ring from n =
  fold (fun node _ -> ring.(node.next)) ring.(from) (1 -- n)

let get_ccw ring from n =
  fold (fun node _ -> ring.(node.prev)) ring.(from) (1 -- n)

let run nplayers nmarbles =
  let ring = Array.create (nmarbles + 1) {id= -1; prev= -1; next= -1} in
  ring.(0) <- {id= 0; prev= 0; next= 0} ;
  let cur = ref 0 in
  let scores = Array.create nplayers 0 in
  for i = 1 to nmarbles do
    if i mod 23 = 0 then (
      let to_remove = get_ccw ring !cur 7 in
      remove_marble ring to_remove.id ;
      let player = i mod nplayers in
      scores.(player) <- scores.(player) + i + to_remove.id ;
      cur := to_remove.next )
    else
      let next = get_cw ring !cur 1 in
      insert_marble ring i next.id ;
      cur := i
  done ;
  Array.fold_left max min_int scores

let () = puts (run 452 (100 * 71250))
