input = File.open('./input') { |f| f.read }.strip.split("\n")

def encoded_size(s)
  s.each_char.reduce(2) do |memo, c|
    case c
    when '\\', '"' then memo += 2
    else memo += 1
    end
  end
end

raw = 0
encoded = 0

input.each do |s|
  raw += s.size
  encoded += encoded_size(s)
end

puts encoded - raw
