open Batteries

let rec first_true vals at =
  if at >= Array.length vals then -1
  else if vals.(at) then at
  else first_true vals (at + 1)

let rec set_interval vals from interval v =
  if from >= Array.length vals then ()
  else (
    vals.(from) <- v ;
    set_interval vals (from + interval) interval v )

let prime_sieve until =
  let vals = Array.create (until + 1) true in
  vals.(0) <- false ;
  vals.(1) <- false ;
  vals.(2) <- true ;
  let rec aux next_prime =
    if next_prime = -1 then ()
    else (
      set_interval vals (next_prime + next_prime) next_prime false ;
      aux (first_true vals (next_prime + 1)) )
  in
  aux 2 ; vals

let () =
  let start = 109300 in
  let finish = 126300 in
  let primes = Array.sub (prime_sieve finish) start (finish - start + 1) in
  let composites =
    Array.fold_lefti
      (fun memo i prime -> if not prime && i mod 17 = 0 then memo + 1 else memo)
      0 primes
  in
  dump composites |> print_endline
