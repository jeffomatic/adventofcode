open Batteries

let sort_chars s =
  s |> String.to_list |> List.sort Char.compare |> String.of_list

let () =
  IO.read_all IO.stdin
  |> Str.split (Str.regexp "\n+")
  |> List.map (fun row ->
         row |> Str.split (Str.regexp " +") |> List.map sort_chars )
  |> List.filter (fun toks ->
         let unique_toks = List.unique toks in
         List.length toks = List.length unique_toks )
  |> List.length |> string_of_int |> print_endline
