open Batteries

type tile = Sand | Clay | Water

type vein = Vert of int * int * int | Horz of int * int * int

let tile_to_string t = match t with Sand -> "." | Clay -> "#" | Water -> "~"

let dimensions grid =
  let h = Array.length grid in
  let w = Array.length grid.(0) in
  (h, w)

let contains grid pos =
  let h, w = dimensions grid in
  let x, y = pos in
  0 <= x && x < w && 0 <= y && y < h

let at grid pos =
  let x, y = pos in
  (grid.(y)).(x)

let parse data =
  let lines = data |> String.trim |> String.split_on_char '\n' in
  let veins =
    lines
    |> List.map (fun line ->
           match line.[0] with
           | 'x' ->
               Scanf.sscanf line "x=%d, y=%d..%d" (fun x y1 y2 ->
                   Vert (x, y1, y2) )
           | 'y' ->
               Scanf.sscanf line "y=%d, x=%d..%d" (fun y x1 x2 ->
                   Horz (y, x1, x2) )
           | _ -> failwith (Printf.sprintf "invalid line: %s" line) )
  in
  let min_x, max_x, min_y, max_y =
    List.fold_left
      (fun accum v ->
        let min_x, max_x, min_y, max_y = accum in
        match v with
        | Vert (x, y1, y2) ->
            let min_x = min min_x x in
            let max_x = max max_x x in
            let min_y = min min_y y1 in
            let max_y = max max_y y2 in
            (min_x, max_x, min_y, max_y)
        | Horz (y, x1, x2) ->
            let min_x = min min_x x1 in
            let max_x = max max_x x2 in
            let min_y = min min_y y in
            let max_y = max max_y y in
            (min_x, max_x, min_y, max_y) )
      (max_int, min_int, max_int, min_int)
      veins
  in
  (* extend x range by one on either side *)
  let min_x = min_x - 1 in
  let max_x = max_x + 1 in
  (* normalize coordinates against min/max ranges *)
  let veins =
    veins
    |> List.map (fun v ->
           match v with
           | Vert (x, y1, y2) -> Vert (x - min_x, y1 - min_y, y2 - min_y)
           | Horz (y, x1, x2) -> Horz (y - min_y, x1 - min_x, x2 - min_x) )
  in
  let h = max_y - min_y + 1 in
  let w = max_x - min_x + 1 in
  let grid = Array.make_matrix h w Sand in
  veins
  |> List.iter (fun v ->
         match v with
         | Vert (x, y1, y2) -> for y = y1 to y2 do (grid.(y)).(x) <- Clay done
         | Horz (y, x1, x2) -> for x = x1 to x2 do (grid.(y)).(x) <- Clay done
     ) ;
  (grid, min_x)

type state = Falling | Left | Right | Settled | Exited

let flow_water grid visited start_x =
  let state = ref Falling in
  let reversed = ref false in
  let pos = ref (start_x, 0) in
  while !state != Settled && !state != Exited do
    let x, y = !pos in
    (visited.(y)).(x) <- true ;
    let down = (x, y + 1) in
    let left = (x - 1, y) in
    let right = (x + 1, y) in
    match !state with
    | Falling -> (
        if not (contains grid down) then state := Exited
        else
          match at grid down with
          | Sand -> pos := down
          | Clay | Water -> state := if Random.bool () then Left else Right )
    | Left ->
        if at grid down = Sand then (
          state := Falling ;
          reversed := false )
        else if at grid left = Sand then pos := left
        else if !reversed then (
          state := Settled ;
          (grid.(y)).(x) <- Water )
        else (
          reversed := true ;
          state := Right )
    | Right ->
        if at grid down = Sand then (
          state := Falling ;
          reversed := false )
        else if at grid right = Sand then pos := right
        else if !reversed then (
          state := Settled ;
          (grid.(y)).(x) <- Water )
        else (
          reversed := true ;
          state := Left )
    | _ -> ()
  done

let grid_to_string grid =
  grid
  |> Array.map (fun row ->
         row |> Array.map tile_to_string |> Array.to_list |> String.join "" )
  |> Array.to_list |> String.join "\n"

let count_water grid =
  Array.fold_left
    (fun accum row ->
      Array.fold_left
        (fun accum tile -> if tile = Water then accum + 1 else accum)
        accum row )
    0 grid

let actual_data = IO.stdin |> IO.read_all

let () =
  let grid, min_x = parse actual_data in
  let start_x = 500 - min_x in
  let h, w = dimensions grid in
  let visited = Array.make_matrix h w false in
  for i = 0 to 100000 do flow_water grid visited start_x done ;
  grid_to_string grid |> print_endline ;
  count_water grid |> dump |> print_endline
