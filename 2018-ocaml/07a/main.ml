open Batteries

let all_steps () = Set.of_enum (Char.range 'A' ~until:'Z')

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

let supplement deps =
  let missing = Set.diff (all_steps ()) (Hashtbl.keys deps |> Set.of_enum) in
  missing |> Set.iter (fun step -> Hashtbl.add deps step Set.empty)

let print_char_list ls =
  ls |> List.map (String.make 1) |> String.join "" |> print_endline

let process deps =
  let rec aux steps =
    if Hashtbl.length deps = 0 then steps
    else
      let remaining =
        deps |> Hashtbl.keys |> List.of_enum |> List.sort compare
      in
      let next =
        remaining
        |> List.find (fun step ->
               Set.diff (Hashtbl.find deps step) (Set.of_list steps)
               |> Set.cardinal = 0 )
      in
      Hashtbl.remove deps next ;
      aux (steps @ [next])
  in
  aux []

let () =
  let input = IO.stdin |> IO.read_all in
  (* let input =
    {|Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.|}
  in *)
  let lines = Str.split (Str.regexp "\n+") input in
  let deps = parse lines in
  supplement deps ;
  let steps = process deps in
  print_char_list steps
