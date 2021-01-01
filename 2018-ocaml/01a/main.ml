open Batteries

let () =
  let input = IO.stdin |> IO.read_all in
  let lines = Str.split (Str.regexp "\n+") input in
  let parsed =
    List.map
      (fun line ->
        Scanf.sscanf line "%c%d" (fun op num ->
            match op with
            | '+' -> num
            | '-' -> -num
            | _ -> failwith ("invalid input: " ^ line) ) )
      lines
  in
  let sum = List.fold_left (fun memo n -> memo + n) 0 parsed in
  dump sum |> print_endline
