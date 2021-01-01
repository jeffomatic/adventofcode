open Batteries

let max_index state =
  let rec aux state next_index max_index max_value =
    if Array.length state = next_index then max_index
    else if state.(next_index) > max_value then
      aux state (next_index + 1) next_index state.(next_index)
    else aux state (next_index + 1) max_index max_value
  in
  aux state 0 0 min_int

let serialize state =
  state |> Array.map string_of_int |> Array.to_list |> String.concat "."

let redistribute state from_index =
  let blocks = state.(from_index) in
  state.(from_index) <- 0 ;
  let rec aux state blocks next_index =
    if blocks = 0 then ()
    else if Array.length state = next_index then aux state blocks 0
    else (
      state.(next_index) <- state.(next_index) + 1 ;
      aux state (blocks - 1) (next_index + 1) )
  in
  aux state blocks (from_index + 1)

let () =
  let input = read_line () in
  let history = Hashtbl.create 1000 in
  let state =
    input
    |> Str.split (Str.regexp " +")
    |> List.map int_of_string |> Array.of_list
  in
  let rec aux state steps =
    let serialized = serialize state in
    if Hashtbl.mem history serialized then
      steps - Hashtbl.find history serialized
    else (
      Hashtbl.add history serialized steps ;
      redistribute state (max_index state) ;
      aux state (steps + 1) )
  in
  aux state 0 |> string_of_int |> print_endline
