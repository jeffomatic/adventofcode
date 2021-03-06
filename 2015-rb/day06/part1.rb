input = File.open('./input') { |f| f.read }.strip.split("\n")

def parse_command(c)
  res = {}
  ['turn on', 'turn off', 'toggle'].each do |prefix|
    if c.start_with?(prefix)
      res[:type] = prefix
      c = c[prefix.size..-1]
      break
    end
  end

  start, finish = c.split(' through ')
  res[:start] = start.split(',').map(&:to_i)
  res[:finish] = finish.split(',').map(&:to_i)

  res
end

grid = 1000.times.map { 1000.times.map { false } }

input.each do |c|
  c = parse_command(c)

  update = nil
  case c[:type]
  when 'turn on'
    update = ->(i, j) { grid[i][j] = true }
  when 'turn off'
    update = ->(i, j) { grid[i][j] = false }
  when 'toggle'
    update = ->(i, j) { grid[i][j] = !grid[i][j] }
  end

  (c[:start][0]..c[:finish][0]).each do |i|
    (c[:start][1]..c[:finish][1]).each do |j|
      update.call(i, j)
    end
  end
end

on = 0
grid.each { |row| row.each { |elem| on +=1 if elem } }
puts on
