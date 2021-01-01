input = File.open('./input') { |f| f.read }.strip.split("\n").map { |l| l.split('x') }

puts input.reduce(0) { |memo, sides|
  sides = sides.map { |d| d.to_i }.sort
  memo += 2 * (sides[0]*sides[1] + sides[0]*sides[2] + sides[1]*sides[2])
  memo += sides.shift * sides.shift
}
