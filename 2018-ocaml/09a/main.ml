open Batteries

let puts x = dump x |> print_endline

type node = {id: int; prev: int; next: int}

let insert_marble ring id after =
  let after_node = Hashtbl.find ring after in
  if after_node.prev = after then (
    Hashtbl.replace ring id {id; prev= after; next= after} ;
    Hashtbl.replace ring after {after_node with prev= id; next= id} )
  else (
    Hashtbl.replace ring after {after_node with next= id} ;
    Hashtbl.replace ring id {id; prev= after; next= after_node.next} ;
    Hashtbl.replace ring after_node.next
      {(Hashtbl.find ring after_node.next) with prev= id} )

let remove_marble ring id =
  let node = Hashtbl.find ring id in
  Hashtbl.remove ring id ;
  Hashtbl.replace ring node.prev
    {(Hashtbl.find ring node.prev) with next= node.next} ;
  Hashtbl.replace ring node.next
    {(Hashtbl.find ring node.next) with prev= node.prev} ;
  ()

let get_cw ring from n =
  fold
    (fun node _ -> Hashtbl.find ring node.next)
    (Hashtbl.find ring from) (1 -- n)

let get_ccw ring from n =
  fold
    (fun node _ -> Hashtbl.find ring node.prev)
    (Hashtbl.find ring from) (1 -- n)

let get_score scores player =
  if Hashtbl.mem scores player then Hashtbl.find scores player else 0

let get_top_score scores = Hashtbl.values scores |> fold max min_int

let run nplayers nmarbles =
  let ring = Hashtbl.create nmarbles in
  Hashtbl.replace ring 0 {id= 0; prev= 0; next= 0} ;
  let cur = ref 0 in
  let scores = Hashtbl.create nplayers in
  let player = ref 0 in
  for i = 1 to nmarbles do
    ( if i mod 23 = 0 then (
        let to_remove = get_ccw ring !cur 7 in
        remove_marble ring to_remove.id ;
        Hashtbl.replace scores !player
          (get_score scores !player + i + to_remove.id) ;
        cur := to_remove.next )
    else
      let next = get_cw ring !cur 1 in
      insert_marble ring i next.id ;
      cur := i ) ;
    player := !player + 1 ;
    if !player >= nplayers then player := 0
  done ;
  get_top_score scores

let test () =
  [ (9, 25, 32)
  ; (10, 1618, 8317)
  ; (17, 1104, 2764)
  ; (21, 6111, 54718)
  ; (30, 5807, 37305) ]
  |> List.iter (fun case ->
         let nplayers, nmarbles, want = case in
         let got = run nplayers nmarbles in
         if got != want then
           failwith
             (Printf.sprintf "Players: %d, marbles %d: got %d want %d" nplayers
                nmarbles got want) )

let () = puts (run 452 71250)
