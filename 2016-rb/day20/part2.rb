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

MAX = 4294967295
merged << [MAX + 1, MAX + 1]

prev = merged.shift
allowed = prev[0]
until merged.empty?
  cur = merged.shift
  allowed += cur[0] - prev[1] - 1
  prev = cur
end

puts allowed
