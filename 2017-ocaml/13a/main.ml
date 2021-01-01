open Batteries

let read_input () = IO.stdin |> IO.read_all |> Str.split (Str.regexp "\n+")

let parse line = Scanf.sscanf line "%d: %d" (fun depth range -> (depth, range))

let () =
  let input = read_input () in
  input |> List.map parse
  |> List.fold_left
       (fun acc layer ->
         let depth, range = layer in
         match depth mod (range + range - 2) with
         | 0 -> acc + (depth * range)
         | _ -> acc )
       0
  |> dump |> print_endline
