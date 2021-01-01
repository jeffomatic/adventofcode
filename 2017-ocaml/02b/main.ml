open Batteries

let is_factor product divisor = product > divisor && product mod divisor = 0

let rec find_factor_pair_for_value v values =
  match values with
  | [] -> []
  | hd :: tl ->
      if is_factor v hd then [v; hd]
      else if is_factor hd v then [hd; v]
      else find_factor_pair_for_value v tl

let rec find_factor_pair values =
  match values with
  | [] -> []
  | hd :: tl ->
    match find_factor_pair_for_value hd tl with
    | [] -> find_factor_pair tl
    | pair -> pair

let () =
  IO.read_all IO.stdin
  |> Str.split (Str.regexp "\n+")
  |> List.map (fun row ->
         let vals =
           Str.split (Str.regexp " +") row |> List.map int_of_string
         in
         match find_factor_pair vals with
         | [a; b] -> a / b
         | _ -> failwith ("Could not find factor pair in " ^ row) )
  |> List.fold_left ( + ) 0 |> string_of_int |> print_endline
