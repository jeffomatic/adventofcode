open Batteries

let parse lines =
  lines
  |> List.map (fun line ->
         Scanf.sscanf line "p=<%d,%d,%d>, v=<%d,%d,%d>, a=<%d,%d,%d>"
           (fun px py pz vx vy vz ax ay az ->
             ((px, py, pz), (vx, vy, vz), (ax, ay, az)) ) )

let distance v =
  let x, y, z = v in
  abs x + abs y + abs z

let min_distance objs =
  objs
  |> List.fold_lefti
       (fun memo n obj ->
         let p, v, a = obj in
         let min_n, min_d = memo in
         if distance p < min_d then (n, distance p) else memo )
       (-1, max_int)

let add_v a b =
  let ax, ay, az = a in
  let bx, by, bz = b in
  (ax + bx, ay + by, az + bz)

let step obj =
  let p, v, a = obj in
  let v = add_v v a in
  let p = add_v p v in
  (p, v, a)

let run objs =
  let rec aux objs n =
    if n = 0 then objs else aux (List.map (fun obj -> step obj) objs) (n - 1)
  in
  min_distance (aux objs 10000)

let () =
  let input = IO.stdin |> IO.read_all in
  (*   let input =
    {|
p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>
|}
  in *)
  let lines = Str.split (Str.regexp "\n+") input in
  let particles = parse lines in
  particles |> run |> dump |> print_endline
