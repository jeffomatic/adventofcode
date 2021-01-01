let () =
  let start_a, start_b = (679, 771) in
  let fac_a, fac_b = (16807, 48271) in
  let mul_a, mul_b = (4, 8) in
  let modulus = 2147483647 in
  let end_at = 5 * 1000 * 1000 in
  let bitmask = 0b1111111111111111 in
  let rec generate prev factor multiple =
    let v = prev * factor mod modulus in
    if v mod multiple = 0 then v else generate v factor multiple
  in
  let rec aux a b matches pairs =
    if pairs = end_at then matches
    else
      let matches =
        if a land bitmask = b land bitmask then matches + 1 else matches
      in
      let a = generate a fac_a mul_a in
      let b = generate b fac_b mul_b in
      aux a b matches (pairs + 1)
  in
  aux start_a start_b 0 0 |> string_of_int |> print_endline
