open Batteries

type state = A | B | C | D | E | F

let array_set arr default_v i v =
  let arr =
    if Array.length arr > i then arr
    else
      let new_arr = Array.create (i + 1) default_v in
      Array.blit arr 0 new_arr 0 (Array.length arr) ;
      new_arr
  in
  arr.(i) <- v ; arr

module Tape = struct
  type t = bool Array.t * int

  type v = Zero | One

  let make () = (Array.make 100 Zero, 0)

  let internal_index i = if i <= 0 then 2 * abs i else (2 * i) - 1

  let get tape i =
    let arr, _ = tape in
    let i = internal_index i in
    if i < Array.length arr then arr.(i) else Zero

  let set tape i v =
    let prev = get tape i in
    let arr, checksum = tape in
    let arr = array_set arr Zero (internal_index i) v in
    let checksum =
      match (prev, v) with
      | Zero, One -> checksum + 1
      | One, Zero -> checksum - 1
      | _, _ -> checksum
    in
    (arr, checksum)

  let checksum tape =
    let _, checksum = tape in
    checksum
end

let step state tape pos =
  let v = Tape.get tape pos in
  match (state, v) with
  | A, Zero -> (B, Tape.set tape pos Tape.One, pos + 1)
  | A, One -> (F, Tape.set tape pos Tape.Zero, pos + 1)
  | B, Zero -> (B, Tape.set tape pos Tape.Zero, pos - 1)
  | B, One -> (C, Tape.set tape pos Tape.One, pos - 1)
  | C, Zero -> (D, Tape.set tape pos Tape.One, pos - 1)
  | C, One -> (C, Tape.set tape pos Tape.Zero, pos + 1)
  | D, Zero -> (E, Tape.set tape pos Tape.One, pos - 1)
  | D, One -> (A, Tape.set tape pos Tape.One, pos + 1)
  | E, Zero -> (F, Tape.set tape pos Tape.One, pos - 1)
  | E, One -> (D, Tape.set tape pos Tape.Zero, pos - 1)
  | F, Zero -> (A, Tape.set tape pos Tape.One, pos + 1)
  | F, One -> (E, Tape.set tape pos Tape.Zero, pos - 1)

let run initial_state steps =
  let rec aux state tape pos steps =
    if steps = 0 then Tape.checksum tape
    else
      let state, tape, pos = step state tape pos in
      aux state tape pos (steps - 1)
  in
  aux initial_state (Tape.make ()) 0 steps

let () = run A 12425180 |> dump |> print_endline
