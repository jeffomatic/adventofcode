open Batteries

let () =
  let input = read_line () in
  let chars = String.to_list input in
  let ints = List.map (fun c -> int_of_char c - int_of_char '0') chars in
  let rec make_pairs ints pair_with_last =
    match ints with
    | [] -> []
    | [a] -> [(a, pair_with_last)]
    | a :: b :: rest -> (a, b) :: make_pairs (b :: rest) pair_with_last
  in
  let pairs = make_pairs ints (List.first ints) in
  let rec sum pairs =
    match pairs with
    | [] -> 0
    | (a, b) :: rest ->
        let pair_val = if a = b then a else 0 in
        pair_val + sum rest
  in
  print_endline (string_of_int @@ sum pairs)
