open Batteries

type nanobot = {pos: int * int * int; radius: int}

let nanobots_from_string data =
  let lines = data |> String.trim |> String.split_on_char '\n' in
  List.map
    (fun line ->
      Scanf.sscanf line "pos=<%d,%d,%d>, r=%d" (fun x y z radius ->
          {pos= (x, y, z); radius} ) )
    lines

let strongest nanobots =
  List.sort (fun a b -> compare b.radius a.radius) nanobots |> List.hd

let dist a b =
  let ax, ay, az = a in
  let bx, by, bz = b in
  abs (ax - bx) + abs (ay - by) + abs (az - bz)

let in_range nanobots pos r =
  List.filter (fun n -> dist n.pos pos <= r) nanobots

let example_data =
  {|
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1
|}

let actual_data = IO.stdin |> IO.read_all

let () =
  let data = actual_data in
  let nanobots = nanobots_from_string data in
  let strongest = strongest nanobots in
  let in_range = in_range nanobots strongest.pos strongest.radius in
  in_range |> List.length |> dump |> print_endline
