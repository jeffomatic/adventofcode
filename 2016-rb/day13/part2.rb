require_relative './min_heap'

FAVORITE_NUMBER = 1364

def has_wall?(x, y)
  v = x*x + 3*x + 2*x*y + y + y*y
  v += FAVORITE_NUMBER
  hamming_weight = v.to_s(2).count("1")
  (hamming_weight/2) == ((hamming_weight-1) / 2)
end

def make_grid(rows, cols)
  grid = []
  rows.times do |i|
    row = []
    grid << row
    cols.times do |j|
      row << !has_wall?(j, i)
    end
  end
  grid
end

def make_graph(rows, cols)
  grid = make_grid(rows, cols)
  verts_by_index = rows.times.map do
    cols.times.map { {neighbors: []} }
  end

  rows.times do |i|
    cols.times do |j|
      next unless grid[i][j] # no neighbors for walls

      vert = verts_by_index[i][j]

      vert[:neighbors] << verts_by_index[i-1][j] if i-1 >= 0 && grid[i-1][j]
      vert[:neighbors] << verts_by_index[i+1][j] if i+1 < rows && grid[i+1][j]
      vert[:neighbors] << verts_by_index[i][j-1] if j-1 >= 0 && grid[i][j-1]
      vert[:neighbors] << verts_by_index[i][j+1] if j+1 < cols && grid[i][j+1]
    end
  end

  verts_by_index
end

def dijkstra(verts, src)
  pq = MinHeap.new

  verts.each do |n|
    distance = 100000000
    if n == src
      distance = 0
    elsif src[:neighbors].include?(n)
      n[:distance] = 1
      n[:prev] = src
      distance = 1
    end

    n[:distance] = distance

    if distance > 0
      pq.insert(n, n[:distance])
    end
  end

  loop do
    v = pq.pop

    return nil if v.nil?
    next if v[:visited]

    v[:visited] = true

    v[:neighbors].each do |n|
      next if n[:visited]
      if n[:distance] > v[:distance] + 1
        n[:distance] = v[:distance] + 1
        n[:prev] = v
        pq.set_priority(n, n[:distance])
      end
    end
  end
end

verts_by_index = make_graph(50, 50)
verts = verts_by_index.reduce([]) { |memo, row| memo + row }
src = verts_by_index[1][1]
dst = verts_by_index[39][31]

dijkstra(verts, src)

puts verts.select {|v| v[:distance] <= 50 }.size
