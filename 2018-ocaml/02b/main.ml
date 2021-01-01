open Batteries

let edit_distance a b =
  let aa = a |> String.to_list |> Array.of_list in
  let bb = b |> String.to_list |> Array.of_list in
  let len = Array.length aa in
  Enum.fold
    (fun tot i -> if aa.(i) != bb.(i) then tot + 1 else tot)
    0
    (0 -- (len - 1))

let rec find_match_with target words =
  match words with
  | [] -> None
  | hd :: tl ->
    match edit_distance target hd with
    | 1 -> Some hd
    | _ -> find_match_with target tl

let rec find_match_in words =
  match words with
  | [] -> failwith "no match found"
  | hd :: tl ->
      let m = find_match_with hd tl in
      match m with None -> find_match_in tl | Some m -> (hd, m)

let exclude_diff a b =
  let aa = a |> String.to_list |> Array.of_list in
  let bb = b |> String.to_list |> Array.of_list in
  let len = Array.length aa in
  Enum.fold
    (fun memo i -> if aa.(i) == bb.(i) then aa.(i) :: memo else memo)
    []
    (0 -- (len - 1))
  |> List.rev |> List.map String.of_char |> String.join ""

let () =
  let input = IO.stdin |> IO.read_all in
  let words = Str.split (Str.regexp "\n+") input in
  let a, b = find_match_in words in
  exclude_diff a b |> print_endline
