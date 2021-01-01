def dragon(a)
  b = a.reverse
  b = b.each_char.map { |c| c == '0' ? '1' : '0' }.join
  a + '0' + b
end

# s must be even
def checksum(s)
  i = 0
  res = ''
  while i < s.size
    res += (s[i] == s[i+1]) ? '1' : '0'
    i += 2
  end
  return checksum(res) if res.size % 2 == 0
  res
end

def disk_data(initial, size)
  res = initial
  while res.size < size
    res = dragon(res)
  end
  res[0...size]
end

puts checksum(disk_data('01111001100111011', 272))
