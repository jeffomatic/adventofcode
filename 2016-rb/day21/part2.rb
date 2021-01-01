input = File.open('./input') { |f| f.read }.strip.split("\n")

s = ARGV[0] || 'fbgdceah'
s = s.each_char.to_a

input.reverse_each do |cmd|
  toks = cmd.split
  if cmd.start_with?('swap position')
    x, y = toks[2].to_i, toks[5].to_i
    s[x], s[y] = s[y], s[x]
  elsif cmd.start_with?('swap letter')
    x, y = toks[2], toks[5]
    s.map! do |c|
      if c == x
        y
      elsif c == y
        x
      else
        c
      end
    end
  elsif cmd.start_with?('rotate based')
    x = toks.last
    i = s.index(x)
    rot = {
      0 => 1,
      1 => 1,
      2 => 6,
      3 => 2,
      4 => 7,
      5 => 3,
      6 => 0,
      7 => 4,
    }
    s.rotate!(rot[i])
  elsif cmd.start_with?('rotate') # left/right
    rots = toks[2].to_i
    rots *= -1 if toks[1] == 'left'
    s.rotate!(rots)
  elsif cmd.start_with?('reverse')
    x, y = toks[2].to_i, toks[4].to_i
    until x >= y
      s[x], s[y] = s[y], s[x]
      x += 1
      y -= 1
    end
  elsif cmd.start_with?('move')
    y, x = toks[2].to_i, toks[5].to_i
    a = s[x]
    if x < y
      (x...y).each { |i| s[i] = s[i+1] }
    else
      (y+1..x).reverse_each { |i| s[i] = s[i-1] }
    end
    s[y] = a
  else
    raise "Unknown command => #{cmd}"
  end
end

puts s.join
