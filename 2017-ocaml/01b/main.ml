open Batteries

let () =
  let input = read_line () in
  (* let input = "1212" in *)
  (* let input = "1221" in *)
  (* let input = "123425" in *)
  (* let input = "123123" in *)
  (* let input = "12131415" in *)
  let chars = Array.of_list (String.to_list input) in
  let ints = Array.map (fun c -> int_of_char c - int_of_char '0') chars in
  let partner arr i =
    let len = Array.length arr in
    let j = (i + (len / 2)) mod len in
    arr.(j)
  in
  let total =
    Array.fold_lefti
      (fun memo i v -> if v = partner ints i then memo + v else memo)
      0 ints
  in
  total |> dump |> print_endline
