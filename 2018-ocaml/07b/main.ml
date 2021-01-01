open Batteries

let cost c = 61 + (Char.code c - Char.code 'A')

let parse lines =
  let deps = Hashtbl.create 100 in
  List.iter
    (fun line ->
      Scanf.sscanf line "Step %c must be finished before step %c can begin."
        (fun dep step ->
          let s =
            if Hashtbl.mem deps step then Hashtbl.find deps step else Set.empty
          in
          Hashtbl.replace deps step (Set.add dep s) ) )
    lines ;
  deps

let supplement deps last =
  let all_steps = Set.of_enum (Char.range 'A' ~until:last) in
  let missing = Set.diff all_steps (Hashtbl.keys deps |> Set.of_enum) in
  missing |> Set.iter (fun step -> Hashtbl.add deps step Set.empty)

let print_char_list ls =
  ls |> List.map (String.make 1) |> String.join "" |> print_endline

let process deps num_workers =
  let tot_steps = Hashtbl.length deps in
  let rec aux t workers finished =
    let workers =
      List.fold_left
        (fun workers worker ->
          let job, _ = worker in
          match job with
          | None -> (
            try
              let claimed_job =
                deps |> Hashtbl.keys |> List.of_enum |> List.sort compare
                |> List.find (fun step ->
                       Set.diff (Hashtbl.find deps step) (Set.of_list finished)
                       |> Set.cardinal = 0 )
              in
              Hashtbl.remove deps claimed_job ;
              (Some claimed_job, cost claimed_job) :: workers
            with Not_found -> worker :: workers )
          | _ -> worker :: workers )
        [] workers
    in
    let workers, finished =
      List.fold_left
        (fun state worker ->
          let workers, finished = state in
          let job, remaining = worker in
          match job with
          | Some job ->
              if remaining = 1 then ((None, -1) :: workers, finished @ [job])
              else ((Some job, remaining - 1) :: workers, finished)
          | _ -> (worker :: workers, finished) )
        ([], finished) workers
    in
    let t = t + 1 in
    if List.length finished = tot_steps then t else aux t workers finished
  in
  let workers =
    Enum.map (fun _ -> (None, -1)) (1 -- num_workers) |> List.of_enum
  in
  aux 0 workers []

let () =
  let input = IO.stdin |> IO.read_all in
  let num_workers = 5 in
  let last = 'Z' in
  (* let input =
    {|Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.|}
  in
  let num_workers = 2 in
  let last = 'F' in *)
  let lines = Str.split (Str.regexp "\n+") input in
  let deps = parse lines in
  supplement deps last ;
  let t = process deps num_workers in
  t |> dump |> print_endline
