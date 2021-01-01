open Batteries

let closest x y points =
  let res, _ =
    List.fold_left
      (fun closest p ->
        let closest_items, closest_d = closest in
        let px, py = p in
        let d = abs (px - x) + abs (py - y) in
        if d = closest_d then (None, d)
        else if d < closest_d then (Some p, d)
        else closest )
      (None, max_int) points
  in
  res

let maybe_append items v = match v with None -> items | Some v -> v :: items

let infinite_area_nodes field =
  let h = Array.length field in
  let w = Array.length field.(0) in
  let nodes = ref [] in
  for i = 0 to h - 1 do
    nodes := maybe_append !nodes (field.(i)).(0) ;
    nodes := maybe_append !nodes (field.(i)).(w - 1)
  done ;
  for j = 1 to w - 2 do
    nodes := maybe_append !nodes (field.(0)).(j) ;
    nodes := maybe_append !nodes (field.(h - 1)).(j)
  done ;
  List.sort_uniq compare !nodes

let count_areas field =
  let tbl = Hashtbl.create 100 in
  let h = Array.length field in
  let w = Array.length field.(0) in
  for i = 0 to h - 1 do
    for j = 0 to w - 1 do
      match (field.(i)).(j) with
      | Some p ->
          if Hashtbl.mem tbl p then
            Hashtbl.replace tbl p (Hashtbl.find tbl p + 1)
          else Hashtbl.add tbl p 1
      | None -> ()
    done
  done ;
  tbl

let remove_keys tbl keys =
  List.iter (fun k -> Hashtbl.remove tbl k) keys ;
  tbl

let () =
  let input = IO.stdin |> IO.read_all in
  (*   let input = {|1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
|} in *)
  let lines = Str.split (Str.regexp "\n+") input in
  let points =
    List.map (fun line -> Scanf.sscanf line "%d, %d" (fun x y -> (x, y))) lines
  in
  let xs =
    List.map
      (fun p ->
        let x, _ = p in
        x )
      points
  in
  let ys =
    List.map
      (fun p ->
        let _, y = p in
        y )
      points
  in
  let min_x = List.fold_left min max_int xs in
  let max_x = List.fold_left max min_int xs in
  let min_y = List.fold_left min max_int ys in
  let max_y = List.fold_left max min_int ys in
  let h = max_y - min_y + 1 in
  let w = max_x - min_x + 1 in
  let field = Array.make_matrix h w None in
  for i = 0 to h - 1 do
    let y = i + min_y in
    for j = 0 to w - 1 do
      let x = j + min_x in
      (field.(i)).(j) <- closest x y points
    done
  done ;
  let areas = count_areas field in
  let blacklist = infinite_area_nodes field in
  let areas = remove_keys areas blacklist in
  let _, area =
    Hashtbl.fold
      (fun p area biggest ->
        let _, biggest_area = biggest in
        if area > biggest_area then (p, area) else biggest )
      areas
      ((-1, -1), min_int)
  in
  area |> dump |> print_endline
