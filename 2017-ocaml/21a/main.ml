open Batteries

let decode s =
  match String.length s with
  | 5 ->
      Scanf.sscanf s "%c%c/%c%c" (fun a b c d ->
          let arr = Array.make_matrix 2 2 '.' in
          (arr.(0)).(0) <- a ;
          (arr.(0)).(1) <- b ;
          (arr.(1)).(0) <- c ;
          (arr.(1)).(1) <- d ;
          arr )
  | 11 ->
      Scanf.sscanf s "%c%c%c/%c%c%c/%c%c%c" (fun a b c d e f g h i ->
          let arr = Array.make_matrix 3 3 '.' in
          (arr.(0)).(0) <- a ;
          (arr.(0)).(1) <- b ;
          (arr.(0)).(2) <- c ;
          (arr.(1)).(0) <- d ;
          (arr.(1)).(1) <- e ;
          (arr.(1)).(2) <- f ;
          (arr.(2)).(0) <- g ;
          (arr.(2)).(1) <- h ;
          (arr.(2)).(2) <- i ;
          arr )
  | 19 ->
      Scanf.sscanf s "%c%c%c%c/%c%c%c%c/%c%c%c%c/%c%c%c%c"
        (fun a b c d e f g h i j k l m n o p ->
          let arr = Array.make_matrix 4 4 '.' in
          (arr.(0)).(0) <- a ;
          (arr.(0)).(1) <- b ;
          (arr.(0)).(2) <- c ;
          (arr.(0)).(3) <- d ;
          (arr.(1)).(0) <- e ;
          (arr.(1)).(1) <- f ;
          (arr.(1)).(2) <- g ;
          (arr.(1)).(3) <- h ;
          (arr.(2)).(0) <- i ;
          (arr.(2)).(1) <- j ;
          (arr.(2)).(2) <- k ;
          (arr.(2)).(3) <- l ;
          (arr.(3)).(0) <- m ;
          (arr.(3)).(1) <- n ;
          (arr.(3)).(2) <- o ;
          (arr.(3)).(3) <- p ;
          arr )
  | _ -> failwith ("invalid encoding: " ^ s)

let encode mat =
  match Array.length mat with
  | 2 ->
      Printf.sprintf "%c%c/%c%c"
        (mat.(0)).(0)
        (mat.(0)).(1)
        (mat.(1)).(0)
        (mat.(1)).(1)
  | 3 ->
      Printf.sprintf "%c%c%c/%c%c%c/%c%c%c"
        (mat.(0)).(0)
        (mat.(0)).(1)
        (mat.(0)).(2)
        (mat.(1)).(0)
        (mat.(1)).(1)
        (mat.(1)).(2)
        (mat.(2)).(0)
        (mat.(2)).(1)
        (mat.(2)).(2)
  | _ -> failwith ("invalid array: " ^ dump mat)

let hflip mat =
  match Array.length mat with
  | 2 ->
      let res = Array.make_matrix 2 2 '.' in
      (res.(0)).(0) <- (mat.(0)).(1) ;
      (res.(0)).(1) <- (mat.(0)).(0) ;
      (res.(1)).(0) <- (mat.(1)).(1) ;
      (res.(1)).(1) <- (mat.(1)).(0) ;
      res
  | 3 ->
      let res = Array.make_matrix 3 3 '.' in
      (res.(0)).(0) <- (mat.(0)).(2) ;
      (res.(0)).(1) <- (mat.(0)).(1) ;
      (res.(0)).(2) <- (mat.(0)).(0) ;
      (res.(1)).(0) <- (mat.(1)).(2) ;
      (res.(1)).(1) <- (mat.(1)).(1) ;
      (res.(1)).(2) <- (mat.(1)).(0) ;
      (res.(2)).(0) <- (mat.(2)).(2) ;
      (res.(2)).(1) <- (mat.(2)).(1) ;
      (res.(2)).(2) <- (mat.(2)).(0) ;
      res
  | _ -> failwith ("invalid array: " ^ dump mat)

let vflip mat =
  match Array.length mat with
  | 2 ->
      let res = Array.make_matrix 2 2 '.' in
      (res.(0)).(0) <- (mat.(1)).(0) ;
      (res.(0)).(1) <- (mat.(1)).(1) ;
      (res.(1)).(0) <- (mat.(0)).(0) ;
      (res.(1)).(1) <- (mat.(0)).(1) ;
      res
  | 3 ->
      let res = Array.make_matrix 3 3 '.' in
      (res.(0)).(0) <- (mat.(2)).(0) ;
      (res.(0)).(1) <- (mat.(2)).(1) ;
      (res.(0)).(2) <- (mat.(2)).(2) ;
      (res.(1)).(0) <- (mat.(1)).(0) ;
      (res.(1)).(1) <- (mat.(1)).(1) ;
      (res.(1)).(2) <- (mat.(1)).(2) ;
      (res.(2)).(0) <- (mat.(0)).(0) ;
      (res.(2)).(1) <- (mat.(0)).(1) ;
      (res.(2)).(2) <- (mat.(0)).(2) ;
      res
  | _ -> failwith ("invalid array: " ^ dump mat)

let rot mat =
  match Array.length mat with
  | 2 ->
      let res = Array.make_matrix 2 2 '.' in
      (res.(0)).(0) <- (mat.(1)).(0) ;
      (res.(0)).(1) <- (mat.(0)).(0) ;
      (res.(1)).(0) <- (mat.(1)).(1) ;
      (res.(1)).(1) <- (mat.(0)).(1) ;
      res
  | 3 ->
      let res = Array.make_matrix 3 3 '.' in
      (res.(0)).(0) <- (mat.(2)).(0) ;
      (res.(0)).(1) <- (mat.(1)).(0) ;
      (res.(0)).(2) <- (mat.(0)).(0) ;
      (res.(1)).(0) <- (mat.(2)).(1) ;
      (res.(1)).(1) <- (mat.(1)).(1) ;
      (res.(1)).(2) <- (mat.(0)).(1) ;
      (res.(2)).(0) <- (mat.(2)).(2) ;
      (res.(2)).(1) <- (mat.(1)).(2) ;
      (res.(2)).(2) <- (mat.(0)).(2) ;
      res
  | _ -> failwith ("invalid array: " ^ dump mat)

let parse lines =
  let ht = Hashtbl.create 2000 in
  let add_rule from into =
    let mat = decode from in
    Hashtbl.add ht (mat |> encode) into ;
    Hashtbl.add ht (mat |> hflip |> encode) into ;
    Hashtbl.add ht (mat |> vflip |> encode) into ;
    let mat = mat |> rot in
    Hashtbl.add ht (mat |> encode) into ;
    Hashtbl.add ht (mat |> hflip |> encode) into ;
    Hashtbl.add ht (mat |> vflip |> encode) into ;
    let mat = mat |> rot in
    Hashtbl.add ht (mat |> encode) into ;
    Hashtbl.add ht (mat |> hflip |> encode) into ;
    Hashtbl.add ht (mat |> vflip |> encode) into ;
    let mat = mat |> rot in
    Hashtbl.add ht (mat |> encode) into ;
    Hashtbl.add ht (mat |> hflip |> encode) into ;
    Hashtbl.add ht (mat |> vflip |> encode) into
  in
  List.iter (fun line -> Scanf.sscanf line "%s => %s" add_rule) lines ;
  ht

let square_iteri size f =
  for i = 0 to size - 1 do for j = 0 to size - 1 do f i j done done

let square_mapi size f =
  let range = List.of_enum (0 -- (size - 1)) in
  List.map (fun i -> List.map (fun j -> f i j) range) range

let mat_fold f memo mat =
  Array.fold_left
    (fun memo rows -> Array.fold_left (fun memo elem -> f memo elem) memo rows)
    memo mat

let subdivide mat =
  let size = Array.length mat in
  let sub_size =
    if size mod 2 = 0 then 2
    else if size mod 3 = 0 then 3
    else failwith ("invalid array size: " ^ dump size)
  in
  square_mapi (size / sub_size) (fun sub_i sub_j ->
      let submat = Array.make_matrix sub_size sub_size '.' in
      square_iteri sub_size (fun i j ->
          (submat.(i)).(j)
          <- (mat.((sub_size * sub_i) + i)).((sub_size * sub_j) + j) ) ;
      submat )

let enhance rules mat = encode mat |> Hashtbl.find rules |> decode

let blit src dst dst_i dst_j =
  square_iteri (Array.length src) (fun i j ->
      (dst.(dst_i + i)).(dst_j + j) <- (src.(i)).(j) )

let step rules mat =
  let enhanced =
    subdivide mat |> List.map (fun row -> row |> List.map (enhance rules))
  in
  let sub_count = List.length enhanced in
  let sub_size = Array.length (enhanced |> List.first |> List.first) in
  let res_size = sub_count * sub_size in
  let res = Array.make_matrix res_size res_size '.' in
  enhanced
  |> List.iteri (fun i rows ->
         rows
         |> List.iteri (fun j sub -> blit sub res (i * sub_size) (j * sub_size))
     ) ;
  res

let count_on mat =
  mat_fold (fun memo c -> if c = '#' then memo + 1 else memo) 0 mat

let run rules start n =
  let rec aux n mat =
    if n = 0 then count_on mat else aux (n - 1) (step rules mat)
  in
  aux n start

let () =
  let input = IO.stdin |> IO.read_all in
  (* let input = {|
../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#
|} in *)
  let rules = input |> Str.split (Str.regexp "\n+") |> parse in
  let start = [|[|'.'; '#'; '.'|]; [|'.'; '.'; '#'|]; [|'#'; '#'; '#'|]|] in
  run rules start 5 |> dump |> print_endline
