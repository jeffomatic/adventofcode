open Batteries

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
                let c = (i, j) in
                if Set.mem c at_least_one then
                  (at_least_one, Set.add c at_least_two)
                else (Set.add c at_least_one, at_least_two) )
              sets
              (x -- (x + w - 1)) )
          sets
          (y -- (y + h - 1)) )
      (Set.empty, Set.empty) claims
  in
  Set.cardinal at_least_two |> dump |> print_endline
