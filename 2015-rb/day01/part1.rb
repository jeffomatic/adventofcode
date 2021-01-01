input = File.open('./input') { |f| f.read }.strip

puts input.each_char.reduce(0) { |memo, c|
  memo += (c == '(') ? 1 : -1
}
