open Batteries

let read_input () = IO.stdin |> IO.read_all |> Str.split (Str.regexp "\n+")

let parse_desc_str desc_str =
  match Str.split (Str.regexp " <-> ") desc_str with
  | [n_str; neighbors_str] ->
      let n = int_of_string n_str in
      let neighbors =
        List.map int_of_string (Str.split (Str.regexp ", ") neighbors_str)
      in
      (n, neighbors)
  | _ -> failwith ("invalid desc str: " ^ desc_str)

let make_adjacency_list descs =
  List.fold_left
    (fun acc desc ->
      let n, neighbors = desc in
      Map.Int.add n neighbors acc )
    Map.Int.empty descs

let make_group seed adjacency_list =
  let rec aux q group =
    match q with
    | [] -> group
    | n :: rest ->
        let neighbors =
          Map.Int.find n adjacency_list |> List.enum |> Set.Int.of_enum
        in
        let diff =
          Set.Int.diff neighbors group |> Set.Int.enum |> List.of_enum
        in
        aux (rest @ diff) (Set.Int.union group neighbors)
  in
  aux [seed] Set.Int.empty

let make_groups adjacency_list =
  List.fold_left
    (fun acc n ->
      if List.exists (fun s -> Set.Int.mem n s) acc then acc
      else make_group n adjacency_list :: acc )
    []
    (Map.Int.keys adjacency_list |> List.of_enum)

let () =
  let input = read_input () in
  input |> List.map parse_desc_str |> make_adjacency_list |> make_groups
  |> List.length |> dump |> print_endline
