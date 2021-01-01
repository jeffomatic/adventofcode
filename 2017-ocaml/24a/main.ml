open Batteries

let parse lines =
  List.fold_lefti
    (fun memo i line ->
      Scanf.sscanf line "%d/%d" (fun a b -> (i, a, b) :: memo) )
    [] lines

let add_part parts_by_size size part =
  if Hashtbl.mem parts_by_size size then
    let parts = Hashtbl.find parts_by_size size in
    Hashtbl.replace parts_by_size size (part :: parts)
  else Hashtbl.add parts_by_size size [part]

let other_port part port =
  let _, a, b = part in
  if a = port then b else a

let collate parts =
  let parts_by_size = Hashtbl.create 20 in
  List.iter
    (fun part ->
      let _, a, b = part in
      add_part parts_by_size a part ;
      if a != b then add_part parts_by_size b part )
    parts ;
  parts_by_size

let filter_parts candidates id_blacklist =
  List.fold_left
    (fun memo part ->
      let id, _, _ = part in
      if List.mem id id_blacklist then memo else part :: memo )
    [] candidates

let rec combinations parts_by_size start_size id_blacklist =
  if not (Hashtbl.mem parts_by_size start_size) then []
  else
    let candidates =
      filter_parts (Hashtbl.find parts_by_size start_size) id_blacklist
    in
    List.fold_left
      (fun memo part ->
        let id, _, _ = part in
        let next_combos =
          combinations parts_by_size
            (other_port part start_size)
            (id :: id_blacklist)
        in
        match next_combos with
        | [] -> [part] :: memo
        | _ ->
            let prefixed_combos =
              List.fold_left
                (fun memo combo -> (part :: combo) :: memo)
                memo next_combos
            in
            [part] :: prefixed_combos )
      [] candidates

let strength combo =
  List.fold_left
    (fun memo part ->
      let _, a, b = part in
      memo + a + b )
    0 combo

let strongest combos =
  List.fold_left (fun memo combo -> max memo (strength combo)) (-1) combos

let () =
  (*   let input = {|
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
|} in *)
  let input = IO.stdin |> IO.read_all in
  let lines = Str.split (Str.regexp "\n+") input in
  let parts_by_size = lines |> parse |> collate in
  let combos = combinations parts_by_size 0 [] in
  dump (strongest combos) |> print_endline
