open Batteries

let () =
  let code =
    IO.read_all IO.stdin
    |> Str.split (Str.regexp "\n+")
    |> List.map (fun v -> int_of_string v)
    |> Array.of_list
  in
  let rec aux pos steps =
    if pos >= Array.length code then steps
    else
      let new_pos = pos + code.(pos) in
      code.(pos) <- code.(pos) + 1 ;
      aux new_pos (steps + 1)
  in
  aux 0 0 |> string_of_int |> print_endline
