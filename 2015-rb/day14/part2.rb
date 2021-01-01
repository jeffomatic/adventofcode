input = File.read('./input') { |f| f.read }.strip.split("\n")
time = 2503

deer = input.map do |line|
  line = line.split(' ')
  {
    name: line[0],
    speed: Integer(line[3]),
    endurance: Integer(line[6]),
    rest: Integer(line[-2]),
  }
end

# deer = [
#   {name: 'Comet', speed: 14, endurance: 10, rest: 127},
#   {name: 'Dancer', speed: 16, endurance: 11, rest: 162},
# ]

def distance(time:, speed:, endurance:, rest:)
  chunks = time / (endurance + rest)
  leftover = time % (endurance + rest)
  leftover = [leftover, endurance].min
  (chunks * speed * endurance) + (leftover * speed)
end

scores = {}
duration = 2503

(1..duration).each do |t|
  max = nil
  leaders = []

  deer.each do |d|
    dist = distance(
      time: t,
      speed: d[:speed],
      endurance: d[:endurance],
      rest: d[:rest]
    )

    if max.nil? || dist > max
      leaders = []
      max = dist
    end

    leaders << d[:name] if dist == max
  end

  leaders.each do |name|
    scores[name] ||= 0
    scores[name] += 1
  end
end

puts scores.to_a.sort { |a, b| a[1] <=> b[1] }.last
