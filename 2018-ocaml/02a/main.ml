open Batteries

let letter_map word =
  List.fold_left
    (fun tbl c ->
      if Hashtbl.mem tbl c then
        let n = Hashtbl.find tbl c + 1 in
        Hashtbl.replace tbl c n
      else Hashtbl.add tbl c 1 ;
      tbl )
    (Hashtbl.create 100) (String.to_list word)

let has_n_of_any tbl n = Enum.exists (fun v -> v == n) (Hashtbl.values tbl)

let count_has_n tbls n =
  List.fold_left
    (fun tot tbl -> if has_n_of_any tbl n then tot + 1 else tot)
    0 tbls

let () =
  let input = IO.stdin |> IO.read_all in
  let lines = Str.split (Str.regexp "\n+") input in
  let tbls = List.map letter_map lines in
  count_has_n tbls 2 * count_has_n tbls 3 |> dump |> print_endline
