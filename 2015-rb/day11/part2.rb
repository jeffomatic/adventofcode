def is_password(chars)
  pairs = 0
  has_straight = false
  current_straight = 1
  prev_char = nil
  pair_ok = true

  chars.each do |c|
    return false if ['i'.ord, 'o'.ord, 'l'.ord].include?(c)

    if prev_char == c && pair_ok
      pairs += 1
      pair_ok = false
    else
      pair_ok = true
    end

    if prev_char && c == prev_char + 1
      current_straight += 1
      has_straight = true if current_straight == 3
    elsif prev_char
      current_straight = 1
    end

    prev_char = c
  end

  pairs >= 2 && has_straight
end

def deserialize(s)
  s.each_char.map { |c| c.ord }
end

def serialize(pw)
  pw.map { |c| c.chr }.join
end

def next_candidate(pw)
  pw = pw.dup
  pw.size.times.to_a.reverse.each do |i|
    if pw[i].nil?
      pw[i] = 'a'.ord
    else
      pw[i] += 1
    end

    if pw[i] == 'z'.ord + 1
      pw[i] = 'a'.ord
      pw.unshift 'a'.ord if i == 0
      next
    end

    if ['i'.ord, 'o'.ord, 'l'.ord].include?(pw[i])
      pw[i] += 1
    end

    break
  end

  pw.shift if pw[0] == nil?
  pw
end

def next_pw(pw)
  loop do
    pw = next_candidate(pw)
    return pw if is_password(pw)
  end
end

puts serialize(next_pw(deserialize('hepxxyzz')))
