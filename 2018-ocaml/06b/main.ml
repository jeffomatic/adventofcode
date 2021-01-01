open Batteries

let close_enough p coords cutoff =
  let rec aux coords tot =
    match coords with
    | [] -> true
    | hd :: tl ->
        let px, py = p in
        let hdx, hdy = hd in
        let tot = tot + abs (hdx - px) + abs (hdy - py) in
        if tot >= cutoff then false else aux tl tot
  in
  aux coords 0

let () =
  let input = IO.stdin |> IO.read_all in
  let cutoff = 10000 in
  (* let input = {|1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
|} in
  let cutoff = 32 in *)
  let lines = Str.split (Str.regexp "\n+") input in
  let coords =
    List.map (fun line -> Scanf.sscanf line "%d, %d" (fun x y -> (x, y))) lines
  in
  let xs =
    List.map
      (fun p ->
        let x, _ = p in
        x )
      coords
  in
  let ys =
    List.map
      (fun p ->
        let _, y = p in
        y )
      coords
  in
  let min_x = List.fold_left min max_int xs in
  let max_x = List.fold_left max min_int xs in
  let min_y = List.fold_left min max_int ys in
  let max_y = List.fold_left max min_int ys in
  let size = ref 0 in
  for y = min_y - cutoff + 1 to max_y + cutoff - 1 do
    for x = min_x - cutoff + 1 to max_x + cutoff - 1 do
      if close_enough (x, y) coords cutoff then size := !size + 1
    done
  done ;
  !size |> dump |> print_endline
