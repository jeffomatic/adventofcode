open Batteries

let rec parse ~garbage_open ~escape ~score ~chars =
  match chars with
  | [] -> score
  | first :: chars ->
      if escape then parse ~garbage_open ~escape:false ~score ~chars
      else if garbage_open then
        match first with
        | '>' -> parse ~garbage_open:false ~escape ~score ~chars
        | '!' -> parse ~garbage_open ~escape:true ~score ~chars
        | _ -> parse ~garbage_open ~escape ~score:(score + 1) ~chars
      else
        match first with
        | '<' -> parse ~garbage_open:true ~escape ~score ~chars
        | '!' -> parse ~garbage_open ~escape:true ~score ~chars
        | _ -> parse ~garbage_open ~escape ~score ~chars

let () =
  let chars = IO.stdin |> IO.read_all |> String.trim |> String.to_list in
  parse ~garbage_open:false ~escape:false ~score:0 ~chars
  |> string_of_int |> print_endline
