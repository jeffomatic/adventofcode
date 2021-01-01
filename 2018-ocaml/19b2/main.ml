open Batteries

let k = 10551292

let factors =
  [ 1
  ; 2
  ; 4
  ; 61
  ; 83
  ; 122
  ; 166
  ; 244
  ; 332
  ; 521
  ; 1042
  ; 2084
  ; 5063
  ; 10126
  ; 20252
  ; 31781
  ; 43243
  ; 63562
  ; 86486
  ; 127124
  ; 172972
  ; 2637823
  ; 5275646
  ; 10551292 ]

let () = List.fold_left ( + ) 0 factors |> dump |> print_endline
