open Batteries

type dir = N | E | S | W

let left facing = match facing with N -> W | W -> S | S -> E | E -> N

let right facing = match facing with N -> E | E -> S | S -> W | W -> N

type coord = int * int

type cart_state = A | B | C

type cart = {id: int; pos: coord; facing: dir; state: cart_state}

let zero_cart = {id= -1; pos= (-1, -1); facing= N; state= A}

let sort_carts carts =
  carts
  |> List.sort (fun a b ->
         let ai, aj = a.pos in
         let bi, bj = b.pos in
         if ai < bi then -1
         else if bi < ai then 1
         else if aj < bj then -1
         else if bj < aj then 1
         else 0 )

let parse_lines lines =
  let lines = List.filter (fun line -> String.length line > 0) lines in
  let h = List.length lines in
  let w =
    List.fold_left
      (fun accum line -> max accum (String.length line))
      min_int lines
  in
  let grid = Array.make_matrix h w ' ' in
  let carts = ref [] in
  for i = 0 to h - 1 do
    let line = List.nth lines i in
    for j = 0 to w - 1 do
      if j >= String.length line then (grid.(i)).(j) <- ' '
      else
        let pos = (i, j) in
        let id = List.length !carts in
        match line.[j] with
        | '^' ->
            (grid.(i)).(j) <- '|' ;
            carts := {zero_cart with id; pos; facing= N} :: !carts
        | '>' ->
            (grid.(i)).(j) <- '-' ;
            carts := {zero_cart with id; pos; facing= E} :: !carts
        | 'v' ->
            (grid.(i)).(j) <- '|' ;
            carts := {zero_cart with id; pos; facing= S} :: !carts
        | '<' ->
            (grid.(i)).(j) <- '-' ;
            carts := {zero_cart with id; pos; facing= W} :: !carts
        | c -> (grid.(i)).(j) <- c
    done
  done ;
  (grid, !carts)

let cart_at pos carts = carts |> List.find (fun c -> c.pos = pos)

let any_cart_at pos carts = carts |> List.exists (fun c -> c.pos = pos)

let move_cart grid cart =
  let i, j = cart.pos in
  match (grid.(i)).(j) with
  | '-' -> (
    match cart.facing with
    | E -> {cart with pos= (i, j + 1)}
    | W -> {cart with pos= (i, j - 1)}
    | _ -> failwith (Printf.sprintf "impossible facing") )
  | '|' -> (
    match cart.facing with
    | N -> {cart with pos= (i - 1, j)}
    | S -> {cart with pos= (i + 1, j)}
    | _ -> failwith (Printf.sprintf "impossible facing") )
  | '/' -> (
    match cart.facing with
    | N -> {cart with pos= (i, j + 1); facing= E}
    | E -> {cart with pos= (i - 1, j); facing= N}
    | S -> {cart with pos= (i, j - 1); facing= W}
    | W -> {cart with pos= (i + 1, j); facing= S} )
  | '\\' -> (
    match cart.facing with
    | N -> {cart with pos= (i, j - 1); facing= W}
    | E -> {cart with pos= (i + 1, j); facing= S}
    | S -> {cart with pos= (i, j + 1); facing= E}
    | W -> {cart with pos= (i - 1, j); facing= N} )
  | '+' -> (
      let cart =
        match cart.state with
        | A -> {cart with facing= left cart.facing; state= B}
        | B -> {cart with state= C}
        | C -> {cart with facing= right cart.facing; state= A}
      in
      match cart.facing with
      | N -> {cart with pos= (i - 1, j)}
      | E -> {cart with pos= (i, j + 1)}
      | S -> {cart with pos= (i + 1, j)}
      | W -> {cart with pos= (i, j - 1)} )
  | c ->
      failwith
        (Printf.sprintf "cart %d at invalid grid tile (%d, %d) %c" cart.id i j
           c)

let tick grid carts step =
  let surviving = ref [] in
  let todo = ref (sort_carts carts) in
  while List.length !todo > 0 do
    let cart = List.hd !todo in
    todo := List.tl !todo ;
    let cart = move_cart grid cart in
    if any_cart_at cart.pos !todo || any_cart_at cart.pos !surviving then (
      let i, j = cart.pos in
      Printf.printf "step %d: collision at %d,%d\n" step j i ;
      todo := List.filter (fun c -> compare c.pos cart.pos != 0) !todo ;
      surviving :=
        List.filter (fun c -> compare c.pos cart.pos != 0) !surviving )
    else surviving := cart :: !surviving
  done ;
  ( if List.length !surviving = 1 then
      let last = List.hd !surviving in
      let i, j = last.pos in
      Printf.printf "last surviving car is at %d,%d\n" j i ) ;
  !surviving

let grid_to_string grid carts =
  grid
  |> Array.mapi (fun i row ->
         row
         |> Array.mapi (fun j c ->
                if any_cart_at (i, j) carts then
                  let cart = cart_at (i, j) carts in
                  match cart.facing with
                  | N -> "^"
                  | E -> ">"
                  | S -> "v"
                  | W -> "<"
                else String.of_char c )
         |> Array.to_list |> String.join "" )
  |> Array.to_list |> String.join "\n"

let test_input =
  {|
/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/
|}

let true_input = IO.stdin |> IO.read_all

let () =
  (* let input = test_input in *)
  let input = true_input in
  let lines = String.split_on_char '\n' input in
  let grid, carts = parse_lines lines in
  fold
    (fun carts step ->
      let carts = tick grid carts step in
      carts )
    carts (1 -- 10830)
  |> ignore
