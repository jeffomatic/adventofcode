open Batteries

type tile = Blank | Vertical | Horizontal | Junction | Letter of char

type dir = N | S | W | E

let longest_line lines =
  List.fold_left (fun memo line -> max memo (String.length line)) 0 lines

let parse lines =
  let h = List.length lines in
  let w = longest_line lines in
  let m = Array.make_matrix h w Blank in
  List.iteri
    (fun i line ->
      String.iteri
        (fun j c ->
          (m.(i)).(j)
          <- ( match c with
             | ' ' -> Blank
             | '|' -> Vertical
             | '-' -> Horizontal
             | '+' -> Junction
             | _ -> Letter c ) )
        line )
    lines ;
  m

let find_start m =
  let j =
    Array.fold_lefti
      (fun memo i t -> match t with Vertical -> i | _ -> memo)
      (-1) m.(0)
  in
  (0, j)

let advance i j d =
  match d with
  | N -> (i - 1, j)
  | S -> (i + 1, j)
  | W -> (i, j - 1)
  | E -> (i, j + 1)

let outside i j h w = i < 0 || h <= i || j < 0 || w <= j

let neighbors i j h w =
  let add_if_inside d i j h w res =
    if outside i j h w then res else (d, i, j) :: res
  in
  []
  |> add_if_inside N (i - 1) j h w
  |> add_if_inside S (i + 1) j h w
  |> add_if_inside W i (j - 1) h w
  |> add_if_inside E i (j + 1) h w

let follow m =
  let h = Array.length m in
  let w = Array.length m.(0) in
  let i, j = find_start m in
  let rec aux i j d letters =
    let next_i, next_j = advance i j d in
    if outside next_i next_j h w then letters
    else
      match (m.(next_i)).(next_j) with
      | Blank -> letters
      | Vertical | Horizontal -> aux next_i next_j d letters
      | Letter c -> aux next_i next_j d (c :: letters)
      | Junction ->
          let d =
            neighbors next_i next_j h w
            |> List.fold_left
                 (fun memo neighbor ->
                   let neighbor_d, neighbor_i, neighbor_j = neighbor in
                   if (i, j) = (neighbor_i, neighbor_j) then memo
                   else
                     let neighbor_type = (m.(neighbor_i)).(neighbor_j) in
                     match neighbor_type with
                     | Letter c -> neighbor_d
                     | _ ->
                       match (neighbor_d, neighbor_type) with
                       | N, Vertical
                        |S, Vertical
                        |E, Horizontal
                        |W, Horizontal ->
                           neighbor_d
                       | _ -> memo )
                 d
          in
          aux next_i next_j d letters
  in
  aux i j S []

let () =
  let input = IO.stdin |> IO.read_all in
  (* let input =
    {|
     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+
|}
  in *)
  let lines = Str.split (Str.regexp "\n+") input in
  let m = parse lines in
  follow m |> List.rev
  |> List.map (fun c -> Char.escaped c)
  |> String.join "" |> print_endline
