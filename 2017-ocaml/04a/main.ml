open Batteries

let () =
  IO.read_all IO.stdin
  |> Str.split (Str.regexp "\n+")
  |> List.filter (fun row ->
         let toks = Str.split (Str.regexp " +") row in
         let unique_toks = List.unique toks in
         List.length toks = List.length unique_toks )
  |> List.length |> string_of_int |> print_endline
