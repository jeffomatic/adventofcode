require 'digest'
require 'set'

def key_candidate(salt, i)
  Digest::MD5.hexdigest(salt + i.to_s)
end

def consecutives(s)
  last = nil
  count = 0
  res = {
    3 => nil,
    5 => Set.new,
  }

  s.each_char do |c|
    if c == last
      count += 1
      res[3] = c if count == 3 && res[3].nil?
      res[5] << c if count == 5
    else
      count = 1
      last = c
    end
  end

  res
end

salt = 'ahsbgdzn'
threes_by_index = {}
fives_by_char = {}

100000.times do |i|
  kc = key_candidate(salt, i)
  c = consecutives(kc)
  threes_by_index[i] = c[3]
  unless c[5].empty?
    c[5].each do |char|
      fives_by_char[char] ||= []
      fives_by_char[char] << i
    end
  end
end

keys = 0
threes_by_index.each do |i_3, c|
  next unless fives_by_char.key?(c)

  fives_by_char[c].each do |i_5|
    next unless i_3 < i_5

    if i_3 + 1000 >= i_5
      keys += 1
      if keys == 64
        puts i_3
        exit
      end
    end

    break
  end
end

