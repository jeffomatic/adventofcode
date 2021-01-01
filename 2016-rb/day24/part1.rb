require 'set'
require_relative '../day13/min_heap'

def stringify(grid, pois)
  grid.map { |row|
    row.map { |v|
      if v.nil?
        '#'
      elsif pois.include?(v[:pos])
        pois.index(v[:pos])
      else
        '.'
      end
    }.join
  }.join("\n")
end

input = File.open('./input').read { |f| f.read }.strip.split("\n")

grid = []
pois = []
input.each_with_index do |line, i|
  row = []
  grid << row

  line.each_char.each_with_index do |c, j|
    if c == '#'
      row << nil
      next
    end

    pos = [i, j]
    v = {pos: pos, neighbors: []}
    row << v

    # Track points of interest
    if c != '.'
      v[:poi_index] = c.to_i
      v[:poi_edges] = []
      pois[c.to_i] = pos
    end
  end
end

# Compose graph
pos_by_degree = 5.times.map { |d| Set.new }
grid.each_with_index do |row, i|
  row.each_with_index do |v, j|
    # Because of the border empty verts, this is the only early-out we need. No
    # further bounds checking is required.
    next unless v

    v[:neighbors] << [i-1, j] if grid[i-1][j]
    v[:neighbors] << [i+1, j] if grid[i+1][j]
    v[:neighbors] << [i, j-1] if grid[i][j-1]
    v[:neighbors] << [i, j+1] if grid[i][j+1]

    d = v[:neighbors].size
    pos_by_degree[d] ||= Set.new
    pos_by_degree[d] << v[:pos]
  end
end

# Prune graph of dead-ends
loop do
  ps = pos_by_degree[1].to_a
  pos_by_degree[1] = Set.new
  removed = false

  ps.each do |p|
    # Skip if this is a point of interest
    if pois.include?(p)
      pos_by_degree[1] << p
      next
    end

    # Remove from grid
    v = grid[p[0]][p[1]]
    grid[p[0]][p[1]] = nil
    removed = true

    # Unlist from neighbors
    v[:neighbors].each do |np|
      n = grid[np[0]][np[1]]
      n[:neighbors].delete(p)
      c = n[:neighbors].size
      pos_by_degree[c+1].delete(n)
      pos_by_degree[c] << np
    end
  end

  # No change means only points of interest remain
  break unless removed
end

# Prune graph of isolated verts
pos_by_degree[0].each do |p|
  grid[p[0]][p[1]] = nil
end

puts "Grid with no dead-ends:"
puts stringify(grid, pois)
puts

INFINITY = 10000000

def dijkstra(grid, src, dst)
  visited = {}
  distances = {}
  pq = MinHeap.new

  grid.each do |row|
    row.each do |v|
      next if v.nil?

      d = INFINITY
      if v == src
        d = 0
      elsif v[:neighbors].include?(src)
        d = 1
      end

      distances[v[:pos]] = d
      pq.insert(v[:pos], d) if d > 0
    end
  end

  loop do
    vpos = pq.pop
    return nil if vpos.nil?
    return nil if distances[vpos] == INFINITY

    next if visited.key?(vpos)
    visited[vpos] = true
    v = grid[vpos[0]][vpos[1]]

    d = distances[vpos] + 1

    v[:neighbors].each do |n|
      next if visited.key?(n)

      return d if n == dst
      next if d >= distances[n]

      distances[n] = d
      pq.set_priority(n, d)
    end
  end
end

pois.each do |apos|
  a = grid[apos[0]][apos[1]]
  pois[a[:poi_index]+1..-1].each do |bpos|
    b = grid[bpos[0]][bpos[1]]
    dist = dijkstra(grid, apos, bpos)
    a[:poi_edges][b[:poi_index]] = dist
    b[:poi_edges][a[:poi_index]] = dist
  end
end

puts "Distance between points of interest:"
pois.each do |p|
  v = grid[p[0]][p[1]]
  puts "#{v[:poi_index]}: #{v[:poi_edges].inspect}"
end
puts

poi_grid = pois.map do |p|
  v = grid[p[0]][p[1]]
  v[:poi_edges].each_with_index.map { |dist, i|
    [i, dist]
  }.reject { |e|
    e[1].nil?
  }.sort { |a, b|
    a[1] <=> b[1]
  }
end

puts "Point-of-interest grid:"
puts poi_grid.inspect
puts

# poi_grid = [[[6, 16], [1, 20], [3, 28], [2, 76], [7, 166], [5, 220], [4, 238]], [[0, 20], [6, 24], [3, 44], [2, 76], [7, 162], [5, 216], [4, 234]], [[1, 76], [0, 76], [6, 88], [3, 92], [7, 230], [5, 284], [4, 302]], [[0, 28], [6, 32], [1, 44], [2, 92], [7, 162], [5, 212], [4, 230]], [[5, 38], [7, 84], [3, 230], [6, 230], [1, 234], [0, 238], [2, 302]], [[4, 38], [7, 66], [3, 212], [6, 212], [1, 216], [0, 220], [2, 284]], [[0, 16], [1, 24], [3, 32], [2, 88], [7, 162], [5, 212], [4, 230]], [[5, 66], [4, 84], [3, 162], [1, 162], [6, 162], [0, 166], [2, 230]]]

q = [{
  path: [0],
  dist: 0,
}]
min_path = nil
min_dist = 100000000

until q.empty? do
  s = q.pop # DFS

  if s[:path].size == poi_grid.size
    if min_dist > s[:dist]
      min_path = s[:path]
      min_dist = s[:dist]
    end
    next
  end

  cur = poi_grid[s[:path].last]
  cur.each do |e|
    next if s[:path].include?(e[0])
    q << {
      path: s[:path] + [e[0]],
      dist: s[:dist] + e[1],
    }
  end
end

puts "Best path:"
puts min_path.inspect
puts "Distance:"
puts min_dist
