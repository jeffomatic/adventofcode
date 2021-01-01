open Batteries

let split_list at lst =
  let rec aux at a b =
    if at = 0 then (List.rev a, b)
    else
      match b with
      | [] -> (List.rev a, [])
      | hd :: tl -> aux (at - 1) (hd :: a) tl
  in
  aux at [] lst

type node = {children: node list; metadata: int list}

let parse toks =
  let rec aux toks =
    match toks with
    | num_children :: num_metadata :: toks ->
        let children, toks =
          Enum.fold
            (fun accum _ ->
              let children, toks = accum in
              let child, toks = aux toks in
              (children @ [child], toks) )
            ([], toks) (1 -- num_children)
        in
        let metadata, toks = split_list num_metadata toks in
        ({children; metadata}, toks)
    | _ -> failwith ("invalid tokens: " ^ dump toks)
  in
  let root, _ = aux toks in
  root

let rec total_metadata root =
  List.fold_left
    (fun tot child -> tot + total_metadata child)
    (List.sum root.metadata) root.children

(* let input = {|2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2|} *)
let input = IO.stdin |> IO.read_all |> String.trim

let data =
  input |> String.split_on_char ' '
  |> List.map (fun s -> Scanf.sscanf s "%d" (fun n -> n))

let () = parse data |> total_metadata |> dump |> print_endline
