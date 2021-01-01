open Batteries

let read_input () = IO.stdin |> IO.read_all |> Str.split (Str.regexp "\n+")

let parse desc_str =
  match Str.split (Str.regexp " <-> ") desc_str with
  | [n_str; neighbors_str] ->
      let n = int_of_string n_str in
      let neighbors =
        List.map int_of_string (Str.split (Str.regexp ", ") neighbors_str)
      in
      (n, neighbors)
  | _ -> failwith ("invalid desc str: " ^ desc_str)

let make_neighbor_list descs =
  List.fold_left
    (fun acc desc ->
      let n, neighbors = desc in
      Map.Int.add n neighbors acc )
    Map.Int.empty descs

let rec in_any_set n sets =
  match sets with
  | [] -> false
  | s :: rest -> if Set.Int.mem n s then true else in_any_set n rest

let rec find_set_containing n sets =
  match sets with
  | [] -> failwith "no set found"
  | s :: rest -> if Set.Int.mem n s then s else find_set_containing n rest

let group_from_seed seed neighbor_list =
  let rec aux q group =
    match q with
    | [] -> group
    | n :: rest ->
        let neighbors =
          Map.Int.find n neighbor_list |> List.enum |> Set.Int.of_enum
        in
        let diff = Set.Int.diff neighbors group in
        let q = rest @ (diff |> Set.Int.enum |> List.of_enum) in
        let group = Set.Int.union group neighbors in
        aux q group
  in
  aux [seed] Set.Int.empty

let make_groups neighbor_list =
  let rec aux nodes groups =
    match nodes with
    | [] -> groups
    | n :: rest ->
        let groups =
          if in_any_set n groups then groups
          else group_from_seed n neighbor_list :: groups
        in
        aux rest groups
  in
  aux (Map.Int.keys neighbor_list |> List.of_enum) []

let () =
  let input = read_input () in
  input |> List.map parse |> make_neighbor_list |> make_groups
  |> find_set_containing 0 |> Set.Int.cardinal |> dump |> print_endline
