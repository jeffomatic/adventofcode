open Batteries

type date = int * int

type action = Sleep of date * int | Wake of date * int | Guard of date * int

type state = Awake | Asleep | Unknown

let next_day date =
  let m, d = date in
  match (m, d) with
  | 1, 31 -> (2, 1)
  | 2, 28 -> (3, 1)
  | 3, 31 -> (4, 1)
  | 4, 30 -> (5, 1)
  | 5, 31 -> (6, 1)
  | 6, 30 -> (7, 1)
  | 7, 31 -> (8, 1)
  | 8, 31 -> (9, 1)
  | 9, 30 -> (10, 1)
  | 10, 31 -> (11, 1)
  | 11, 30 -> (12, 1)
  | _, _ -> (m, d + 1)

let parse_line line =
  if
    not
      (Str.string_match
         (Str.regexp
            "\\[1518-\\([0-9][0-9]\\)-\\([0-9][0-9]\\) \
             \\([0-9][0-9]\\):\\([0-9][0-9]\\)\\] \\([^)]+\\)")
         line 0)
  then failwith ("invalid line " ^ line) ;
  let m = Int.of_string (Str.matched_group 1 line) in
  let d = Int.of_string (Str.matched_group 2 line) in
  let hr = Int.of_string (Str.matched_group 3 line) in
  let min = Int.of_string (Str.matched_group 4 line) in
  let date = (m, d) in
  match Str.matched_group 5 line with
  | "wakes up" -> Wake (date, min)
  | "falls asleep" -> Sleep (date, min)
  | s ->
      Scanf.sscanf s "Guard #%d" (fun guard ->
          let date = match hr with 23 -> next_day date | _ -> date in
          Guard (date, guard) )

let make_guards_by_date actions =
  let tbl = Hashtbl.create 100 in
  List.iter
    (fun action ->
      match action with Guard (date, id) -> Hashtbl.add tbl date id | _ -> ()
      )
    actions ;
  tbl

let sleepwake_min action =
  match action with
  | Sleep (date, min) -> min
  | Wake (date, min) -> min
  | _ -> failwith "invalid action"

let add_sleepwake tbl action =
  let date, min =
    match action with
    | Sleep (date, min) -> (date, min)
    | Wake (date, min) -> (date, min)
    | _ -> failwith "invalid action"
  in
  if Hashtbl.mem tbl date then
    Hashtbl.replace tbl date (action :: Hashtbl.find tbl date)
  else Hashtbl.add tbl date [action]

let make_sleepwake_by_date actions =
  let tbl = Hashtbl.create 100 in
  List.iter
    (fun action ->
      match action with
      | Guard _ -> ()
      | sleepwake -> add_sleepwake tbl sleepwake )
    actions ;
  tbl

let make_state_by_min actions =
  let state_by_min = Array.create 60 Unknown in
  List.iter
    (fun action ->
      match action with
      | Sleep (_, min) -> state_by_min.(min) <- Asleep
      | Wake (_, min) -> state_by_min.(min) <- Awake
      | _ -> failwith "invalid action" )
    actions ;
  ignore
    (Array.fold_lefti
       (fun prev i state ->
         let state = match state with Unknown -> prev | _ -> state in
         state_by_min.(i) <- state ; state )
       Awake state_by_min) ;
  state_by_min

let make_state_by_date sleepwake_by_date =
  let tbl = Hashtbl.create 100 in
  Hashtbl.iter
    (fun date actions -> Hashtbl.add tbl date (make_state_by_min actions))
    sleepwake_by_date ;
  tbl

let make_guard_states guards_by_date state_by_date =
  let tbl = Hashtbl.create 100 in
  Hashtbl.iter
    (fun date guard ->
      let state_by_min =
        if Hashtbl.mem state_by_date date then Hashtbl.find state_by_date date
        else Array.make 60 Awake
      in
      if Hashtbl.mem tbl guard then
        Hashtbl.replace tbl guard (state_by_min :: Hashtbl.find tbl guard)
      else Hashtbl.add tbl guard [state_by_min] )
    guards_by_date ;
  tbl

let count_asleep state_by_min =
  Array.fold_left
    (fun tot state -> match state with Asleep -> tot + 1 | _ -> tot)
    0 state_by_min

let find_sleepiest_min states =
  let asleep_by_min = Array.make 60 0 in
  List.iter
    (fun state ->
      Array.iteri
        (fun min state ->
          match state with
          | Asleep -> asleep_by_min.(min) <- asleep_by_min.(min) + 1
          | _ -> () )
        state )
    states ;
  Array.fold_lefti
    (fun sleepiest min tot ->
      let _, best_tot = sleepiest in
      if tot > best_tot then (min, tot) else sleepiest )
    (-1, -1) asleep_by_min

let find_best_guard guard_states =
  Hashtbl.fold
    (fun guard states best ->
      let _, _, best_tot = best in
      let guard_min, guard_tot = find_sleepiest_min states in
      if guard_tot > best_tot then (guard, guard_min, guard_tot) else best )
    guard_states (-1, -1, -1)

let () =
  let input = IO.stdin |> IO.read_all in
  (*   let input =
    {|[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
|}
  in *)
  let lines = Str.split (Str.regexp "\n+") input in
  let actions = List.map parse_line lines in
  let guards_by_date = make_guards_by_date actions in
  let sleepwake_by_date = make_sleepwake_by_date actions in
  let state_by_date = make_state_by_date sleepwake_by_date in
  let guard_states = make_guard_states guards_by_date state_by_date in
  let guard, min, _ = find_best_guard guard_states in
  guard * min |> dump |> print_endline
