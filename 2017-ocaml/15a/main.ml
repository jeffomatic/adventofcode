let () =
  let start_a, start_b = (679, 771) in
  let fac_a, fac_b = (16807, 48271) in
  let modulus = 2147483647 in
  let end_at = 40 * 1000 * 1000 in
  let bitmask = 0b1111111111111111 in
  let rec aux a b matches pairs =
    if pairs = end_at then matches
    else
      let matches =
        if a land bitmask = b land bitmask then matches + 1 else matches
      in
      let a = a * fac_a mod modulus in
      let b = b * fac_b mod modulus in
      aux a b matches (pairs + 1)
  in
  aux start_a start_b 0 0 |> string_of_int |> print_endline
