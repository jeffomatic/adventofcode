open Batteries

type vec = int * int

type particle = {p: vec; v: vec}

let particle_from_string s =
  if
    not
      (Str.string_match
         (Str.regexp
            "position=< *\\([^,]+\\), +\\([^)]+\\) velocity=< *\\([^,]+\\), \
             +\\([^)]+\\)")
         s 0)
  then failwith ("invalid particle: " ^ s) ;
  let matches =
    fold (fun matches n -> matches @ [Str.matched_group n s]) [] (1 -- 4)
  in
  let vals = matches |> List.map (fun s -> Scanf.sscanf s "%d" (fun v -> v)) in
  {p= (List.nth vals 0, List.nth vals 1); v= (List.nth vals 2, List.nth vals 3)}

let step particle =
  let px, py = particle.p in
  let vx, vy = particle.v in
  {particle with p= (px + vx, py + vy)}

let step_all particles =
  Array.iteri (fun i particle -> particles.(i) <- step particle) particles

let range particles =
  Array.fold_left
    (fun range particle ->
      let min_x, max_x, min_y, max_y = range in
      let px, py = particle.p in
      (min min_x px, max max_x px, min min_y py, max max_y py) )
    (max_int, min_int, max_int, min_int)
    particles

let print particles =
  let min_x, max_x, min_y, max_y = range particles in
  let h = max_y - min_y + 1 in
  let w = max_x - min_x + 1 in
  let buf = Array.make_matrix h w '.' in
  Array.iter
    (fun particle ->
      let px, py = particle.p in
      (buf.(py - min_y)).(px - min_x) <- '#' )
    particles ;
  for i = 0 to h - 1 do
    for j = 0 to w - 1 do print_char (buf.(i)).(j) done ;
    print_newline ()
  done

let simulate particles print_start print_end =
  for i = 1 to 20000 do
    step_all particles ;
    if print_start <= i && i <= print_end then (
      Printf.printf "%d\n" i ; print particles )
  done

let input = IO.stdin |> IO.read_all |> String.trim

let print_start, print_end = (10117, 10117)

let lines = String.split_on_char '\n' input

let particles = lines |> List.map particle_from_string |> Array.of_list

let () = simulate particles print_start print_end
