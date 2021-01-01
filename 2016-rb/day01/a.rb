input = 'L5, R1, R3, L4, R3, R1, L3, L2, R3, L5, L1, L2, R5, L1, R5, R1, L4, R1, R3, L4, L1, R2, R5, R3, R1, R1, L1, R1, L1, L2, L1, R2, L5, L188, L4, R1, R4, L3, R47, R1, L1, R77, R5, L2, R1, L2, R4, L5, L1, R3, R187, L4, L3, L3, R2, L3, L5, L4, L4, R1, R5, L4, L3, L3, L3, L2, L5, R1, L2, R5, L3, L4, R4, L5, R3, R4, L2, L1, L4, R1, L3, R1, R3, L2, R1, R4, R5, L3, R5, R3, L3, R4, L2, L5, L1, L1, R3, R1, L4, R3, R3, L2, R5, R4, R1, R3, L4, R3, R3, L2, L4, L5, R1, L4, L5, R4, L2, L1, L3, L3, L5, R3, L4, L3, R5, R4, R2, L4, R2, R3, L3, R4, L1, L3, R2, R1, R5, L4, L5, L5, R4, L5, L2, L4, R4, R4, R1, L3, L2, L4, R3'

transitions = {
  N: {
    L: :W,
    R: :E,
  },
  S: {
    L: :E,
    R: :W,
  },
  E: {
    L: :N,
    R: :S,
  },
  W: {
    L: :S,
    R: :N,
  },
}

x = 0
y = 0
orientation = :N

input.split(', ').each do |item|
  turn = item[0].to_sym
  steps = item[1..-1].to_i
  orientation = transitions[orientation][turn]

  case orientation
  when :N then y += steps
  when :S then y -= steps
  when :E then x += steps
  when :W then x -= steps
  end
end

res = x.abs + y.abs
puts res
