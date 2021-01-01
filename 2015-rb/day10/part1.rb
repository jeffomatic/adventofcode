input = '1113122113'

def encode(s)
  current = nil
  count = 0
  res = ''

  s.each_char do |c|
    if current.nil?
      current = c
      count = 1
      next
    end

    if c != current
      res += count.to_s + current
      current = c
      count = 1
      next
    end

    count += 1
  end

  res + count.to_s + current
end

40.times do |i|
 input = encode(input)
end

puts input.size
