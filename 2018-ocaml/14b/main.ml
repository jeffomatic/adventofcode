open Batteries

let digits s = if s < 10 then [s] else [1; s - 10]

let advance cur steps length =
  if cur + steps < length then cur + steps
  else
    let steps = steps - (length - cur) in
    steps mod length

let step recipes len a b =
  let sum = recipes.(a) + recipes.(b) in
  let len = ref len in
  List.iter
    (fun d ->
      recipes.(!len) <- d ;
      len := !len + 1 )
    (digits sum) ;
  let a = advance a (1 + recipes.(a)) !len in
  let b = advance b (1 + recipes.(b)) !len in
  (recipes, !len, a, b)

let rec go recipes len a b pattern =
  let rec aux recipes len a b need =
    let prev_len = len in
    let recipes, len, a, b = step recipes len a b in
    if len - prev_len = 2 then
      match need with
      | [x] ->
          if x = recipes.(len - 2) then prev_len + 1 - List.length pattern
          else aux recipes len a b pattern
      | [x; y] ->
          if x = recipes.(len - 2) && y = recipes.(len - 1) then
            len - List.length pattern
          else aux recipes len a b pattern
      | x :: y :: tl ->
          if x = recipes.(len - 2) && y = recipes.(len - 1) then
            aux recipes len a b tl
          else if List.hd pattern = recipes.(len - 1) then
            aux recipes len a b (List.tl pattern)
          else aux recipes len a b pattern
      | _ -> failwith "impossible"
    else
      match need with
      | [x] ->
          if x = recipes.(len - 1) then len - List.length pattern
          else aux recipes len a b pattern
      | x :: tl ->
          if x = recipes.(len - 1) then aux recipes len a b tl
          else aux recipes len a b pattern
      | _ -> failwith "impossible"
  in
  aux recipes len a b pattern

let () =
  let input = "920831" in
  let need =
    input |> String.to_list |> List.map (fun v -> Char.code v - Char.code '0')
  in
  let recipes = Array.make 100000000 0 in
  recipes.(0) <- 3 ;
  recipes.(1) <- 7 ;
  go recipes 2 0 1 need |> dump |> print_endline
