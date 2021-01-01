input = File.readlines('./input').map do |line|
  line.split.map { |edge| edge.to_i }
end

def triangle?(edges)
  return false if edges[0] + edges[1] <= edges[2]
  return false if edges[1] + edges[2] <= edges[0]
  return false if edges[2] + edges[0] <= edges[1]
  true
end

possible = 0
input.each do |edges|
  possible += 1 if triangle?(edges)
end

puts possible
