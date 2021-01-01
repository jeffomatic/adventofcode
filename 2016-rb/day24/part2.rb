poi_grid = [[[6, 16], [1, 20], [3, 28], [2, 76], [7, 166], [5, 220], [4, 238]], [[0, 20], [6, 24], [3, 44], [2, 76], [7, 162], [5, 216], [4, 234]], [[1, 76], [0, 76], [6, 88], [3, 92], [7, 230], [5, 284], [4, 302]], [[0, 28], [6, 32], [1, 44], [2, 92], [7, 162], [5, 212], [4, 230]], [[5, 38], [7, 84], [3, 230], [6, 230], [1, 234], [0, 238], [2, 302]], [[4, 38], [7, 66], [3, 212], [6, 212], [1, 216], [0, 220], [2, 284]], [[0, 16], [1, 24], [3, 32], [2, 88], [7, 162], [5, 212], [4, 230]], [[5, 66], [4, 84], [3, 162], [1, 162], [6, 162], [0, 166], [2, 230]]]

q = [{
  path: [0],
  dist: 0,
}]
min_path = nil
min_dist = 100000000

until q.empty? do
  s = q.pop # DFS
  cur = poi_grid[s[:path].last]

  if s[:path].size == poi_grid.size + 1
    if min_dist > s[:dist]
      min_path = s[:path]
      min_dist = s[:dist]
    end
  elsif s[:path].size == poi_grid.size
    e = cur.find { |i| i[0] == 0 }
    q << {
      path: s[:path] + [0],
      dist: s[:dist] + e[1],
    }
  else
    cur.each do |e|
      next if s[:path].include?(e[0])
      q << {
        path: s[:path] + [e[0]],
        dist: s[:dist] + e[1],
      }
    end
  end
end

puts min_path.inspect
puts min_dist
