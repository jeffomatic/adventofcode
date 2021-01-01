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

let filter chars low_letter =
  let rev_res =
    List.fold_left
      (fun memo c ->
        if Char.code c = Char.code low_letter then memo
        else if
          Char.code c = Char.code low_letter + Char.code 'A' - Char.code 'a'
        then memo
        else c :: memo )
      [] chars
  in
  List.rev rev_res

let discover_shortest chars =
  Enum.fold
    (fun shortest int_letter ->
      let filtered = filter chars (Char.chr int_letter) in
      let reduced = reduce_all filtered in
      min (List.length reduced) shortest )
    max_int
    (Char.code 'a' -- Char.code 'z')

let () =
  let input = IO.stdin |> IO.read_all |> String.trim in
  (* let input = "dabAcCaCBAcCcaDA" in *)
  let chars = input |> String.to_list in
  let res = discover_shortest chars in
  res |> dump |> print_endline
