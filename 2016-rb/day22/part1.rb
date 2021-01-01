input = File.open('./input') { |f| f.read }.strip.split("\n")
input = input[2..-1] # remove useless lines

nodes = {}

input.each do |line|
  toks = line.split
  matches = toks[0].match(/x(\d+)-y(\d+)/)
  x, y = matches[1].to_i, matches[2].to_i
  size = toks[1].chomp('T').to_i
  used = toks[2].chomp('T').to_i
  avail = toks[3].chomp('T').to_i

  id = [x, y]
  nodes[id] = {size: size, used: used, avail: avail}
end

viable = 0
nodes.keys.each do |a|
  used = nodes[a][:used]
  next if used == 0
  nodes.keys.each do |b|
    next if a == b
    next if used > nodes[b][:avail]
    viable += 1
  end
end

puts viable
