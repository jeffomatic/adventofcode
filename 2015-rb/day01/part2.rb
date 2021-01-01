input = File.open('./input') { |f| f.read }.strip

loc = 0
input.each_char.each_with_index do |c, i|
  loc += (c == '(') ? 1 : -1
  if loc == -1
    puts i + 1
    exit
  end
end
