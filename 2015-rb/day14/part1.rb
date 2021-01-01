input = File.read('./input') { |f| f.read }.strip.split("\n")
time = 2503

deer = input.map do |line|
  line = line.split(' ')
  {
    speed: Integer(line[3]),
    endurance: Integer(line[6]),
    rest: Integer(line[-2]),
  }
end

def distance(time:, speed:, endurance:, rest:)
  chunks = time / (endurance + rest)
  leftover = time % (endurance + rest)
  leftover = [leftover, endurance].min
  (chunks * speed * endurance) + (leftover * speed)
end

best = nil
deer.each do |d|
  current = distance(
    time: time,
    speed: d[:speed],
    endurance: d[:endurance],
    rest: d[:rest],
  )
  best = current if best.nil? || current > best
end

puts best
