open Batteries

let () =
  IO.read_all IO.stdin
  |> Str.split (Str.regexp "\n+")
  |> List.map (fun row ->
         let vals =
           Str.split (Str.regexp " +") row |> List.map int_of_string
         in
         let min = List.fold_left min max_int vals in
         let max = List.fold_left max min_int vals in
         max - min )
  |> List.fold_left ( + ) 0 |> string_of_int |> print_endline
