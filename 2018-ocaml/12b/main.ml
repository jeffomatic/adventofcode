open Batteries

let putv v = dump v |> print_endline

let array_set arr default_v i v =
  let arr =
    if Array.length arr > i then arr
    else
      let new_arr = Array.create (i + 1) default_v in
      Array.blit arr 0 new_arr 0 (Array.length arr) ;
      new_arr
  in
  arr.(i) <- v ; arr

module State = struct
  type t = bool Array.t * int * int

  let make () = (Array.make 1 false, max_int, min_int)

  let min_true t =
    let _, min_true, _ = t in
    min_true

  let max_true t =
    let _, _, max_true = t in
    max_true

  let internal_index i = if i <= 0 then 2 * abs i else (2 * i) - 1

  let get t i =
    let arr, _, _ = t in
    let i = internal_index i in
    if i < Array.length arr then arr.(i) else false

  let set t i v =
    let arr, min_true, max_true = t in
    let arr = array_set arr false (internal_index i) v in
    if v then (arr, min min_true i, max max_true i)
    else if min_true = i then
      let rec aux j = if arr.(internal_index j) then j else aux (j + 1) in
      (arr, aux (i + 1), max_true)
    else if max_true = i then
      let rec aux j = if arr.(internal_index j) then j else aux (j - 1) in
      (arr, aux (i - 1), max_true)
    else (arr, min_true, max_true)

  let to_string t =
    let _, min_true, max_true = t in
    fold (fun s i -> s ^ if get t i then "#" else ".") "" (min_true -- max_true)

  let score t =
    let arr, min_true, max_true = t in
    fold (fun s i -> if get t i then s + i else s) 0 (min_true -- max_true)
end

let parse_notes raw =
  let notes = Hashtbl.create 100 in
  List.iter
    (fun line ->
      Scanf.sscanf line "%c%c%c%c%c => %c" (fun p0 p1 p2 p3 p4 res ->
          Hashtbl.add notes
            (p0 == '#', p1 == '#', p2 == '#', p3 == '#', p4 == '#')
            (res == '#') ) )
    raw ;
  notes

let parse_input lines =
  match lines with
  | initial_state :: blank :: notes ->
      let raw =
        Scanf.sscanf initial_state "initial state: %s" (fun raw -> raw)
      in
      let state =
        String.fold_lefti
          (fun state i c -> State.set state i (c == '#'))
          (State.make ()) raw
      in
      let notes = parse_notes notes in
      (state, notes)
  | _ -> failwith "invalid input"

let step state notes =
  let next = ref (State.make ()) in
  for i = State.min_true state - 5 to State.max_true state + 5 do
    let p =
      ( State.get state (i - 2)
      , State.get state (i - 1)
      , State.get state i
      , State.get state (i + 1)
      , State.get state (i + 2) )
    in
    let res = if Hashtbl.mem notes p then Hashtbl.find notes p else false in
    next := State.set !next i res
  done ;
  !next

let input = IO.stdin |> IO.read_all |> String.trim

let test_input =
  {|initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #|}

let () =
  let lines = String.split_on_char '\n' input in
  let state, notes = parse_input lines in
  ignore
    (fold
       (fun state i ->
         let before = State.score state in
         let state = step state notes in
         let after = State.score state in
         Printf.printf "%d-%d: %d (%d)\n" i (i + 1) after (after - before) ;
         state )
       state (0 -- 1000))
