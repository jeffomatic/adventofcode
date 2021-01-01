# b + c = 100
# max(
#   (-b + 2c) *
#   (-2b + 3c) *
#   (6b - 2c) *
#   (3b - c)
# )
#
# sp + pb + fr + su = 100
# max(
#   (5sp - pb  - su) *
#   (-sp + 3pb -fr) *
#   (4fr) *
#   (2su)
# )

t = 100
max = -1000000
best = nil
(0..t).each do |b|
  c = t - b
  score = [-b + (2*c), 0].max * [(-2*b) + (3*c), 0].max * [((6*b) - (2*c)), 0].max * [((3*b) - c), 0].max
puts [b, c].inspect + ' ' + score.to_s
  if score > max
    max = score
    best = [b, c]
  end
end

puts max
puts best.inspect

t = 100
max = -1000000
best = nil
(0..t).each do |sp|
  (0..(t-sp)).each do |pb|
    (0..(t-sp-pb)).each do |fr|
      su = t - sp - pb - fr
      score = [(5*sp) - pb  - su, 0].max *
        [-sp + (3*pb) - fr, 0].max *
        (4*fr) *
        (2*su)

      cals = (5*sp) + pb + (6*fr) + (8*su)
      next unless cals == 500

      if score > max
        max = score
        best = [sp, pb, fr, su]
      end
    end
  end
end

puts max
puts best.inspect
