(* inspired by the following solution: https://pastebin.com/e7LdSNQe *)

open Batteries

type vec = int * int * int

let vadd a b =
  let ax, ay, az = a in
  let bx, by, bz = b in
  (ax + bx, ay + by, az + bz)

let vsub a b =
  let ax, ay, az = a in
  let bx, by, bz = b in
  (ax - bx, ay - by, az - bz)

let vscale v k =
  let vx, vy, vz = v in
  (k * vx, k * vy, k * vz)

let vscale_div v k =
  let vx, vy, vz = v in
  (vx / k, vy / k, vz / k)

let vmag v =
  let x, y, z = v in
  abs x + abs y + abs z

let vmin a b =
  let ax, ay, az = a in
  let bx, by, bz = b in
  (min ax bx, min ay by, min az bz)

let vmax a b =
  let ax, ay, az = a in
  let bx, by, bz = b in
  (max ax bx, max ay by, max az bz)

type bot = {pos: vec; radius: int}

let bot_from_string s =
  Scanf.sscanf s "pos=<%d,%d,%d>, r=%d" (fun x y z radius ->
      {pos= (x, y, z); radius} )

let bots_from_string data =
  let lines = data |> String.trim |> String.split_on_char '\n' in
  List.map bot_from_string lines

let best_sector bots range scale =
  let min_corner, max_corner = range in
  let max_i, max_j, max_k = vscale_div (vsub max_corner min_corner) scale in
  fold
    (fun accum i ->
      fold
        (fun accum j ->
          fold
            (fun accum k ->
              let p = vadd min_corner (vscale (i, j, k) scale) in
              let in_range =
                List.fold_left
                  (fun accum b ->
                    let dist = vmag (vsub p b.pos) in
                    if (dist - b.radius) / scale <= 0 then accum + 1 else accum
                    )
                  0 bots
              in
              let _, best_in_range = accum in
              if in_range > best_in_range then (p, in_range) else accum )
            accum (0 -- max_k) )
        accum (0 -- max_j) )
    ((0, 0, 0), 0)
    (0 -- max_i)

let pow2_greater_than n =
  let rec aux res = if res > n then res else aux (res * 2) in
  aux 1

let execute bots =
  let min_corner, max_corner =
    List.fold_left
      (fun accum b ->
        let min_corner, max_corner = accum in
        (vmin min_corner b.pos, vmax max_corner b.pos) )
      ((0, 0, 0), (0, 0, 0))
      bots
  in
  let spanx, _, _ = vsub max_corner min_corner in
  let scale = pow2_greater_than spanx in
  let rec aux range scale =
    let sector, in_range = best_sector bots range scale in
    if scale = 1 then (sector, in_range)
    else
      let min_corner = vsub sector (scale, scale, scale) in
      let max_corner = vadd sector (scale, scale, scale) in
      aux (min_corner, max_corner) (scale / 2)
  in
  aux (min_corner, max_corner) scale

let ex1 =
  {|
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5
|}

(* This example demonstrates flaws in the solution above. The correct result is
   1002, from the point (-1002, 0, 0). However, the early phases of solution
   will bias toward a large number of non-overlapping volumes that are captured
   when the scale is broad. In other words, the solution works for the given
   input because the input happens to be structured in a specific way. *)
let ex2 =
  {|
pos=<-1000,0,0>, r=2
pos=<-1002,0,0>, r=2
pos=<100,0,0>, r=1
pos=<103,0,0>, r=1
pos=<106,0,0>, r=1
|}

let actual_data = IO.stdin |> IO.read_all

let bots = bots_from_string actual_data

let () =
  let best_sector, in_range = execute bots in
  Printf.printf "Best sector: %s\nDistance: %d\nBots in range: %d\n"
    (dump best_sector) (vmag best_sector) in_range
