open Batteries

let insert_after pos v c =
  let rec aux pos v c prefix =
    match pos with
    | -1 -> prefix @ (v :: c)
    | _ ->
      match c with
      | [] -> prefix @ [v]
      | first :: rest -> aux (pos - 1) v rest (prefix @ [first])
  in
  aux pos v c []

let steps step_size stop_after =
  let rec aux i pos c =
    if i > stop_after then c
    else
      let c = insert_after pos i c in
      aux (i + 1) ((pos + 1 + step_size) mod (i + 1)) c
  in
  aux 1 0 [0]

let () =
  let step_size = 304 in
  let stop_after = 2017 in
  let c = steps step_size stop_after in
  let pos_last = List.index_of stop_after c in
  match pos_last with
  | None -> failwith "shouldn't get here"
  | Some n ->
      let pos_next = (n + 1) mod (stop_after + 1) in
      List.at c pos_next |> string_of_int |> print_endline
