open Batteries

let () =
  let input = IO.stdin |> IO.read_all in
  let lines = Str.split (Str.regexp "\n+") input in
  let freq_list =
    List.map
      (fun line ->
        Scanf.sscanf line "%c%d" (fun op num ->
            match op with
            | '+' -> num
            | '-' -> -num
            | _ -> failwith ("invalid input: " ^ line) ) )
      lines
  in
  let rec aux freq remaining seen =
    match remaining with
    | [] -> aux freq freq_list seen
    | n :: rest ->
        let freq = freq + n in
        if Set.Int.mem freq seen then freq
        else aux freq rest (Set.Int.add freq seen)
  in
  aux 0 freq_list (Set.Int.add 0 Set.Int.empty) |> dump |> print_endline
