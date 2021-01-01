input = File.open('./input') { |f| f.read }.strip

visited = {
  [0, 0] => true
}
pos = [0, 0]

input.each_char do |c|
  case c
  when '<' then pos[0] -= 1
  when '>' then pos[0] += 1
  when '^' then pos[1] -= 1
  when 'v' then pos[1] += 1
  else raise "Invalid direction: #{c}"
  end

  visited[pos] = true
end

puts visited.size
