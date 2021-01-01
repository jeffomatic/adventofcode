input = File.open('./input') { |f| f.read }.strip

visited = {
  [0, 0].inspect => true
}

a = [0, 0]
b = [0, 0]
is_a = true

input.each_char do |c|
  pos = is_a ? a : b
  is_a = !is_a

  case c
  when '<' then pos[0] -= 1
  when '>' then pos[0] += 1
  when '^' then pos[1] -= 1
  when 'v' then pos[1] += 1
  else raise "Invalid direction: #{c}"
  end

  visited[pos.inspect] = true
end

puts visited.size
