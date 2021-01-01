input = '^.....^.^^^^^.^..^^.^.......^^..^^^..^^^^..^.^^.^.^....^^...^^.^^.^...^^.^^^^..^^.....^.^...^.^.^^.^'

rows = [
  input.each_char.map { |c| c == '^' }
]

39.times do |i|
  prev_row = rows[i]
  rows << prev_row.size.times.map do |j|
    left = (j == 0) ? false : prev_row[j-1]
    center = prev_row[j]
    right = (j == prev_row.size - 1) ? false : prev_row[j+1]

    (left && center && !right) ||
    (center && right && !left) ||
    (left && !center && !right) ||
    (right && !left && !center)
  end
end

total = 0
rows.each do |r|
  r.each do |t|
    total += 1 unless t
  end
end

puts total
