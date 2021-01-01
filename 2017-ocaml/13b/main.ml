open Batteries

let read_input () = IO.stdin |> IO.read_all |> Str.split (Str.regexp "\n+")

let parse line = Scanf.sscanf line "%d: %d" (fun depth range -> (depth, range))

let trip_unsafe delay layers =
  List.exists
    (fun layer ->
      let depth, range = layer in
      (delay + depth) mod (range + range - 2) = 0 )
    layers

let find_safe_trip layers =
  let rec aux delay =
    if trip_unsafe delay layers then aux (delay + 1) else delay
  in
  aux 0

let () =
  let input = read_input () in
  input |> List.map parse |> find_safe_trip |> dump |> print_endline
