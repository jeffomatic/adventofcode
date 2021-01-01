open Batteries

let rec parse ~depth ~garbage_open ~escape ~score ~chars =
  match chars with
  | [] -> score
  | first :: chars ->
      if escape then parse ~depth ~garbage_open ~escape:false ~score ~chars
      else if garbage_open then
        match first with
        | '>' -> parse ~depth ~garbage_open:false ~escape ~score ~chars
        | '!' -> parse ~depth ~garbage_open ~escape:true ~score ~chars
        | _ -> parse ~depth ~garbage_open ~escape ~score ~chars
      else
        match first with
        | '{' -> parse ~depth:(depth + 1) ~garbage_open ~escape ~score ~chars
        | '}' ->
            parse ~depth:(depth - 1) ~garbage_open ~escape
              ~score:(score + depth) ~chars
        | '<' -> parse ~depth ~garbage_open:true ~escape ~score ~chars
        | '>' -> parse ~depth ~garbage_open ~escape ~score ~chars
        | '!' -> parse ~depth ~garbage_open ~escape:true ~score ~chars
        | _ -> parse ~depth ~garbage_open ~escape ~score ~chars

let () =
  let chars = IO.stdin |> IO.read_all |> String.trim |> String.to_list in
  parse ~depth:0 ~garbage_open:false ~escape:false ~score:0 ~chars
  |> string_of_int |> print_endline
