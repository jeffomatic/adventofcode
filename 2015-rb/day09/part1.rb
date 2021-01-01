input = File.open('./input') { |f| f.read }.strip.split("\n")

$distances = {}
all_nodes = []

input.each do |s|
  s =~ /(\w+) to (\w+) = (\d+)/
  a = Regexp.last_match(1)
  b = Regexp.last_match(2)
  dist = Integer(Regexp.last_match(3))
  $distances[[a, b]] = dist
  $distances[[b, a]] = dist
  all_nodes << a
  all_nodes << b
end

all_nodes = all_nodes.uniq

$memo = {}
def hamiltonian(nodes)
  return {route: nodes, total: $distances[nodes]} if nodes.size == 2

  sorted = nodes.sort
  return $memo[sorted] if $memo[sorted]

  best = nil
  sorted.each do |node|
    others = sorted - [node]
    best_others = hamiltonian(others)
    a = $distances[[node, best_others[:route].first]]
    b = $distances[[node, best_others[:route].last]]

    candidate = {}
    if a < b
      candidate[:route] = [node] + best_others[:route]
      candidate[:total] = a + best_others[:total]
    else
      candidate[:route] = best_others[:route] + [node]
      candidate[:total] = b + best_others[:total]
    end

    if best.nil? || candidate[:total] < best[:total]
      best = candidate
    end
  end

  $memo[sorted] = best
end

puts hamiltonian(all_nodes)
