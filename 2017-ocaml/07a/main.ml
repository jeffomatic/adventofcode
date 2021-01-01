open Batteries
module StringSet = Set.Make (String)

type node = {name: string; weight: int; children: string list}

let name_and_weight prefix = Scanf.sscanf prefix "%s (%d)" (fun n w -> (n, w))

let rec collect_names nodes set =
  match nodes with
  | [] -> set
  | first :: rest -> set |> StringSet.add first.name |> collect_names rest

let rec collect_children nodes set =
  match nodes with
  | [] -> set
  | first :: rest ->
      let rec aux children set =
        match children with
        | [] -> set
        | first :: rest -> set |> StringSet.add first |> aux rest
      in
      set |> aux first.children |> collect_children rest

let () =
  let nodes =
    IO.stdin |> IO.read_all
    |> Str.split (Str.regexp "\n+")
    |> List.map (fun line ->
           match Str.split (Str.regexp " -> ") line with
           | [prefix] ->
               let name, weight = name_and_weight prefix in
               {name; weight; children= []}
           | [prefix; children] ->
               let name, weight = name_and_weight prefix in
               {name; weight; children= Str.split (Str.regexp ", ") children}
           | _ -> failwith @@ "Invalid line: " ^ line )
  in
  let all_names = collect_names nodes StringSet.empty in
  let child_names = collect_children nodes StringSet.empty in
  StringSet.diff all_names child_names
  |> StringSet.elements |> List.iter print_endline
