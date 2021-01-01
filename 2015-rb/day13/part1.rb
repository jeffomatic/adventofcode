input = File.open('./input') { |f| f.read }.strip.split("\n")

$deltas = {}
people = []

input.each do |line|
  toks = line.chomp('.').split(' ')
  people << toks[0]
  pair = [toks[0], toks[-1]].sort
  change = Integer(toks[3]) * (toks[2] == 'gain' ? 1 : -1)

  $deltas[pair] ||= 0
  $deltas[pair] += change
end

people.uniq!

def perms(items)
  case items.size
  when 0 then return []
  when 1 then return [items.dup]
  end

  res = []
  items.each do |item|
    perms(items - [item]).each do |sub|
      res << [item] + sub
    end
  end

  res
end

def seatings(items)
  perms(items[1..-1]).map { |rest| [items.first] + rest }
end

def pairs(seating)
  seating.each_with_index.reduce([]) do |memo, (item, i)|
    memo << [item, seating[(i + 1) % seating.size]].sort
  end
end

def seating_score(seating)
  pairs(seating).reduce(0) do |memo, pair|
    memo += $deltas[pair]
  end
end

best_score = nil
seatings(people).each do |seating|
  score = seating_score(seating)
  best_score = score if best_score.nil? || score > best_score
end

puts best_score
