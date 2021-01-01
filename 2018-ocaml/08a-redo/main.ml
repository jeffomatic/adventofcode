open Batteries

let scan data =
  let rec aux i =
    let nchildren = data.(i) in
    let nmeta = data.(i + 1) in
    let v, i =
      fold
        (fun accum _ ->
          let v, i = accum in
          let vchild, i = aux i in
          (v + vchild, i) )
        (0, i + 2)
        (1 -- nchildren)
    in
    fold
      (fun accum _ ->
        let v, i = accum in
        (v + data.(i), i + 1) )
      (v, i) (1 -- nmeta)
  in
  let v, _ = aux 0 in
  v

(* let input = {|2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2|} *)
let input = IO.stdin |> IO.read_all |> String.trim

let data =
  input |> String.split_on_char ' '
  |> List.map (fun s -> Scanf.sscanf s "%d" (fun n -> n))
  |> Array.of_list

let () = scan data |> dump |> print_endline
