open Batteries

let parse lines =
  lines
  |> List.map (fun line ->
         Scanf.sscanf line "p=<%d,%d,%d>, v=<%d,%d,%d>, a=<%d,%d,%d>"
           (fun px py pz vx vy vz ax ay az ->
             ((px, py, pz), (vx, vy, vz), (ax, ay, az)) ) )

let add_v a b =
  let ax, ay, az = a in
  let bx, by, bz = b in
  (ax + bx, ay + by, az + bz)

let step obj =
  let p, v, a = obj in
  let v = add_v v a in
  let p = add_v p v in
  (p, v, a)

let eq_v a b =
  let ax, ay, az = a in
  let bx, by, bz = b in
  ax = bx && ay = by && az = bz

let rec remove_dupes objs =
  match objs with
  | [] -> []
  | first :: rest ->
      let first_p, _, _ = first in
      let without_first =
        List.fold_left
          (fun memo obj ->
            let obj_p, _, _ = obj in
            if eq_v first_p obj_p then memo else obj :: memo )
          [] rest
      in
      if List.length rest > List.length without_first then
        remove_dupes without_first
      else first :: remove_dupes without_first

let run objs =
  let rec aux n objs =
    if n = 0 then objs
    else objs |> List.map (fun obj -> step obj) |> remove_dupes |> aux (n - 1)
  in
  objs |> aux 1000 |> List.length

let () =
  let input = IO.stdin |> IO.read_all in
  (* let input =
    {|
p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>
|}
  in *)
  let lines = Str.split (Str.regexp "\n+") input in
  let particles = parse lines in
  particles |> run |> dump |> print_endline
