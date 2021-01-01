open Batteries

let rec row_in_set set x y w =
  match w with
  | 0 -> false
  | _ -> if Set.mem (x, y) set then true else row_in_set set (x + 1) y (w - 1)

let rec rect_in_set set x y w h =
  if h = 0 then false
  else if row_in_set set x y w then true
  else rect_in_set set x (y + 1) w (h - 1)

let rec find_nonoverapping overlap_locs claims =
  match claims with
  | [] -> failwith "no nonoverlapping claim found"
  | claim :: rest ->
      let id, x, y, w, h = claim in
      if not (rect_in_set overlap_locs x y w h) then id
      else find_nonoverapping overlap_locs rest

let () =
  let input = IO.stdin |> IO.read_all in
  let lines = Str.split (Str.regexp "\n+") input in
  let claims =
    List.map
      (fun line ->
        Scanf.sscanf line "#%d @ %d,%d: %dx%d" (fun id x y w h ->
            (id, x, y, w, h) ) )
      lines
  in
  let _, at_least_two =
    List.fold_left
      (fun sets claim ->
        let id, x, y, w, h = claim in
        Enum.fold
          (fun sets i ->
            Enum.fold
              (fun sets j ->
                let at_least_one, at_least_two = sets in
                let loc = (j, i) in
                if Set.mem loc at_least_one then
                  (at_least_one, Set.add loc at_least_two)
                else (Set.add loc at_least_one, at_least_two) )
              sets
              (x -- (x + w - 1)) )
          sets
          (y -- (y + h - 1)) )
      (Set.empty, Set.empty) claims
  in
  find_nonoverapping at_least_two claims |> dump |> print_endline
