open Batteries

module KnotHash = struct
  let range n =
    let arr = Array.make n 0 in
    for i = 0 to n - 1 do arr.(i) <- i done ;
    arr

  let loop_copy src start len =
    let dst = Array.make len 0 in
    let src_len = Array.length src in
    let dst_i = ref 0 in
    (* Copy to the end of src *)
    for src_i = start to min src_len (start + len) - 1 do
      dst.(!dst_i) <- src.(src_i) ;
      incr dst_i
    done ;
    (* Copy any overflow starting from the beginning of src *)
    let overflow = len - (src_len - start) in
    for src_i = 0 to overflow - 1 do
      dst.(!dst_i) <- src.(src_i) ;
      incr dst_i
    done ;
    dst

  (* Write all of src into dst, starting at dst.(start), and looping to the
   * beginning of src if necessary
   *)
  let loop_blit src dst start =
    let src_len = Array.length src in
    let dst_len = Array.length dst in
    let src_i = ref 0 in
    (* Write to the end of dst *)
    for dst_i = start to min dst_len (start + src_len) - 1 do
      dst.(dst_i) <- src.(!src_i) ;
      incr src_i
    done ;
    (* Write any overflow starting from the begining of dst *)
    let overflow = src_len - (dst_len - start) in
    for dst_i = 0 to overflow - 1 do
      dst.(dst_i) <- src.(!src_i) ;
      incr src_i
    done ;
    ()

  let reverse arr =
    for i = 0 to (Array.length arr / 2) - 1 do
      let j = Array.length arr - i - 1 in
      let tmp = arr.(i) in
      arr.(i) <- arr.(j) ;
      arr.(j) <- tmp
    done ;
    ()

  let sparse_hash input =
    let arr = range 256 in
    let pos = ref 0 in
    let skip = ref 0 in
    for _ = 1 to 64 do
      Array.iter
        (fun len ->
          let span = loop_copy arr !pos len in
          reverse span ;
          loop_blit span arr !pos ;
          pos := (!pos + len + !skip) mod 256 ;
          incr skip )
        input
    done ;
    arr

  let rec partition arr partition_size =
    if Array.length arr = partition_size then [arr]
    else
      let halfsize = Array.length arr / 2 in
      let front = Array.sub arr 0 halfsize in
      let back = Array.sub arr halfsize halfsize in
      partition front partition_size @ partition back partition_size

  let multi_xor arr = Array.fold_left (fun acc v -> acc lxor v) 0 arr

  let dense_hash sparse_hash =
    partition sparse_hash 16 |> List.map multi_xor |> Array.of_list

  let hash input =
    let input = String.to_list input |> List.map Char.code in
    let suffix = [17; 31; 73; 47; 23] in
    let input = input @ suffix in
    sparse_hash (Array.of_list input) |> dense_hash
end

let make_row preimage =
  let hash = KnotHash.hash preimage in
  Array.fold_left
    (fun acc chunk ->
      let bits =
        Enum.fold
          (fun acc n ->
            let bit = chunk land (1 lsl n) > 0 in
            bit :: acc )
          [] (0 -- 7)
      in
      acc @ bits )
    [] hash
  |> Array.of_list

let make_grid input =
  Array.map
    (fun v -> Printf.sprintf "%s-%d" input v |> make_row)
    (Array.of_enum (0 -- 127))

let print_grid g =
  Array.iter
    (fun row ->
      Array.iter (fun v -> print_string (if v then "#" else ".")) row ;
      print_newline () )
    g

let check_grid p grid =
  let i, j = p in
  i >= 0 && j >= 0 && (grid.(i)).(j)

let north_of p =
  let i, j = p in
  (i - 1, j)

let west_of p =
  let i, j = p in
  (i, j - 1)

let point_key ij =
  let i, j = ij in
  string_of_int i ^ "-" ^ string_of_int j

let region_of p regions =
  let p2r, _ = regions in
  Map.String.find (point_key p) p2r

let add_region p regions =
  let p2r, r2p = regions in
  let r = Map.String.cardinal p2r in
  let k = point_key p in
  let p2r = Map.String.add k r p2r in
  let r2p = Map.Int.add r [k] r2p in
  (p2r, r2p)

let merge_regions from into regions =
  if from = into then regions
  else
    let p2r, r2p = regions in
    let from_points = Map.Int.find from r2p in
    let to_points = Map.Int.find into r2p in
    let r2p = Map.Int.remove from r2p in
    let r2p = Map.Int.add into (to_points @ from_points) r2p in
    let p2r =
      List.fold_left (fun acc p -> Map.String.add p into acc) p2r from_points
    in
    (p2r, r2p)

let check_merge from into grid regions =
  if not (check_grid into grid) then regions
  else merge_regions (region_of from regions) (region_of into regions) regions

let matrix_fold_lefti f acc mat =
  Array.fold_lefti
    (fun acc i row -> Array.fold_lefti (fun acc j v -> f acc (i, j) v) acc row)
    acc mat

let make_regions grid =
  matrix_fold_lefti
    (fun acc p populated ->
      if not populated then acc
      else
        add_region p acc
        |> check_merge p (north_of p) grid
        |> check_merge p (west_of p) grid )
    (Map.String.empty, Map.Int.empty)
    grid

let count_regions regions =
  let _, r2p = regions in
  Map.Int.cardinal r2p

let () =
  let input = "amgozmfv" in
  let grid = make_grid input in
  print_grid grid ;
  make_regions grid |> count_regions |> dump |> print_endline
