open Batteries

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
let loop_write src dst start =
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

let run input n =
  let arr = range n in
  let rec aux input i skip =
    match input with
    | [] -> ()
    | len :: rest ->
        let span = loop_copy arr i len in
        reverse span ;
        loop_write span arr i ;
        aux rest ((i + len + skip) mod n) (skip + 1)
  in
  aux input 0 0 ; arr

let read_input () =
  IO.stdin |> IO.read_all |> String.trim
  |> Str.split (Str.regexp ",")
  |> List.map int_of_string

let () =
  let input = read_input () in
  let res = run input 256 in
  dump res |> print_endline ;
  Printf.printf "%d\n" (res.(0) * res.(1))
