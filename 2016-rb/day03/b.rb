input = File.readlines('./input').map do |line|
  line.split.map { |edge| edge.to_i }
end

groups = [[]]
input.each do |line|
  groups << [] if groups.last.size == 3
  groups.last << line
end

candidates = []
groups.each do |g|
  candidates << [g[0][0], g[1][0], g[2][0]]
  candidates << [g[0][1], g[1][1], g[2][1]]
  candidates << [g[0][2], g[1][2], g[2][2]]
end

def triangle?(edges)
  return false if edges[0] + edges[1] <= edges[2]
  return false if edges[1] + edges[2] <= edges[0]
  return false if edges[2] + edges[0] <= edges[1]
  true
end

possible = 0
candidates.each do |edges|
  possible += 1 if triangle?(edges)
end

puts possible
