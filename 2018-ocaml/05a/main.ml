open Batteries

let reduce chars =
  let res, last =
    List.fold_left
      (fun memo c ->
        let res, prev = memo in
        match prev with
        | None -> (res, Some c)
        | Some prev ->
            if
              abs (Char.code prev - Char.code c)
              = abs (Char.code 'A' - Char.code 'a')
            then (res, None)
            else (prev :: res, Some c) )
      ([], None) chars
  in
  match last with None -> List.rev res | Some last -> List.rev (last :: res)

let reduce_all chars =
  let rec aux chars =
    let res = reduce chars in
    if chars = res then res else aux res
  in
  aux chars

let () =
  let input = IO.stdin |> IO.read_all |> String.trim in
  (* let input = "dabAcCaCBAcCcaDA" in *)
  let chars = input |> String.to_list in
  let res = reduce_all chars in
  (* List.map String.of_char |> String.join "" |> print_endline *)
  List.length res |> dump |> print_endline
