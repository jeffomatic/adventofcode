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
      List.iter
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

  let dense_hash sparse_hash = partition sparse_hash 16 |> List.map multi_xor

  let hash input =
    let suffix = [17; 31; 73; 47; 23] in
    let input = input @ suffix in
    sparse_hash input |> dense_hash
end

let rec hamming_weight n =
  if n = 0 then 0 else (n land 1) + hamming_weight (n lsr 1)

let calc_used preimage =
  List.map Char.code (String.to_list preimage)
  |> KnotHash.hash
  |> List.fold_left (fun acc v -> acc + hamming_weight v) 0

let () =
  let input = "amgozmfv" in
  let preimages =
    List.map
      (fun v -> Printf.sprintf "%s-%d" input v)
      (List.of_enum (0 -- 127))
  in
  List.fold_left (fun acc s -> acc + calc_used s) 0 preimages
  |> dump |> print_endline
