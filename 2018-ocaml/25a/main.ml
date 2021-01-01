open Batteries

let point_from_string s =
  Scanf.sscanf s "%d,%d,%d,%d" (fun x y z w -> (x, y, z, w))

let points_from_string s =
  let lines = s |> String.trim |> String.split_on_char '\n' in
  List.map point_from_string lines

let point_to_string p =
  let x, y, z, w = p in
  Printf.sprintf "(%d, %d, %d, %d)" x y z w

let pointset_to_string c =
  c |> Set.to_list |> List.map point_to_string |> String.join " "

let dist a b =
  let ax, ay, az, aw = a in
  let bx, by, bz, bw = b in
  abs (ax - bx) + abs (ay - by) + abs (az - bz) + abs (aw - bw)

let close_points points =
  let res = Hashtbl.create (List.length points) in
  List.iter
    (fun a ->
      let close =
        List.fold_left
          (fun accum b ->
            if a = b then accum
            else if dist a b > 3 then accum
            else Set.add b accum )
          Set.empty points
      in
      Hashtbl.add res a close )
    points ;
  res

let find_constellations points =
  let close_points = close_points points in
  let constellations, _ =
    Hashtbl.fold
      (fun p close_to_p accum ->
        let constellations, processed = accum in
        if Set.mem p processed then (constellations, processed)
        else
          let rec aux constellation q =
            if Set.is_empty q then constellation
            else
              let constellation = Set.union constellation q in
              let next_q =
                Set.fold
                  (fun other_p q ->
                    Set.union q (Hashtbl.find close_points other_p) )
                  q Set.empty
              in
              let next_q = Set.diff next_q constellation in
              aux constellation next_q
          in
          let constellation = aux (Set.add p Set.empty) close_to_p in
          let processed = Set.union processed constellation in
          (constellation :: constellations, processed) )
      close_points ([], Set.empty)
  in
  constellations

let example1 =
  {|
0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0
|}

let example2 =
  {|
-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0
|}

let example3 =
  {|
1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2
|}

let example4 =
  {|
1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2
|}

let actual = IO.stdin |> IO.read_all

let data = actual

let () =
  let points = points_from_string data in
  let constellations = find_constellations points in
  List.length constellations |> dump |> print_endline
