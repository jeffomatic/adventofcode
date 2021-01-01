require_relative './min_heap'

class String
  # colorization
  def colorize(color_code)
    "\e[#{color_code}m#{self}\e[0m"
  end

  def red
    colorize(31)
  end

  def green
    colorize(32)
  end

  def yellow
    colorize(33)
  end

  def blue
    colorize(34)
  end

  def pink
    colorize(35)
  end

  def light_blue
    colorize(36)
  end
end

FAVORITE_NUMBER = 1364

def has_wall?(x, y)
  v = x*x + 3*x + 2*x*y + y + y*y
  v += FAVORITE_NUMBER
  hamming_weight = v.to_s(2).count("1")
  (hamming_weight/2) == ((hamming_weight-1) / 2)
end

def draw(rows, cols, colors = {})
  res = "  "
  cols.times do |j|
    v = j % 10
    if v == 0
      res += (j/10).to_s.green
    else
      res += v.to_s
    end
  end

  res += "\n"

  grid = make_grid(rows, cols)

  rows.times do |i|
    v = i % 10
    if v == 0
      res += (i/10).to_s.green
    else
      res += v.to_s
    end

    res += ' '

    cols.times do |j|
      p = grid[i][j] ? '.' : '#'
      if colors[[j, i]]
        p = p.send(colors[[j, i]])
      end
      res += p
    end

    res += "\n"
  end

  res
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

def dijkstra(verts, src, dst)
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
        return n[:distance] if n == dst
        pq.set_priority(n, n[:distance])
      end
    end
  end
end

puts draw(50, 50, [1, 1] => :red, [31, 39] => :red)

verts_by_index = make_graph(50, 50)
verts = verts_by_index.reduce([]) { |memo, row| memo + row }
src = verts_by_index[1][1]
dst = verts_by_index[39][31]

puts dijkstra(verts, src, dst)
