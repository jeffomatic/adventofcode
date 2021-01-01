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

let rec go recipes len a b n =
  let rec aux recipes len a b =
    if len >= n then recipes
    else
      let recipes, len, a, b = step recipes len a b in
      aux recipes len a b
  in
  aux recipes len a b

let () =
  let input = 920831 in
  let need_length = input + 10 in
  let recipes = Array.make (need_length + 1) 0 in
  recipes.(0) <- 3 ;
  recipes.(1) <- 7 ;
  let recipes = go recipes 2 0 1 (input + 10) in
  let answer = Array.sub recipes input 10 in
  answer |> Array.map String.of_int |> Array.to_list |> String.join ""
  |> print_endline
