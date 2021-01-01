input = File.open('./input') { |f| f.read }.strip.split("\n")

ranges = input.map { |line|
  line.split('-').map(&:to_i)
}.sort { |a, b|
  a[0] <=> b[0]
}

prev = ranges.shift
merged = [prev]

ranges.each do |r|
  if r[0] - 1 <= prev[1]
    prev[1] = [prev[1], r[1]].max
    next
  end

  merged << r
  prev = r
end

puts merged[0][1] + 1
