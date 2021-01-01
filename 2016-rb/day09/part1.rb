input = File.open('./input') { |f| f.read }.strip

def read_until(stream, char)
  read = []
  loop do
    c = stream.shift
    return read if c == char
    read << c
  end
end

def read(stream, n)
  read = []
  n.times do
    read << stream.shift
  end
  read
end

stream = input.each_char.to_a
res = []

until stream.empty?
  c = stream.shift

  if c != '('
    res << c
    next
  end

  size = read_until(stream, 'x').join.to_i
  repeat = read_until(stream, ')').join.to_i
  seq = read(stream, size)
  res += seq * repeat
end

puts res.size
