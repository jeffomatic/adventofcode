input = '^.....^.^^^^^.^..^^.^.......^^..^^^..^^^^..^.^^.^.^....^^...^^.^^.^...^^.^^^^..^^.....^.^...^.^.^^.^'

prev_row = input.each_char.map { |c| c == '^' }
total = prev_row.reduce(0) { |memo, trap| memo += 1 if !trap; memo }

(400000 - 1).times do |i|
  new_row = prev_row.size.times.map do |j|
    left = (j == 0) ? false : prev_row[j-1]
    center = prev_row[j]
    right = (j == prev_row.size - 1) ? false : prev_row[j+1]

    (left && center && !right) ||
    (center && right && !left) ||
    (left && !center && !right) ||
    (right && !left && !center)
  end

  total = new_row.reduce(total) { |memo, trap| memo += 1 if !trap; memo }
  prev_row = new_row
end

puts total
