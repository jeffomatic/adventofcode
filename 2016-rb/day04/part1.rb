raw_input = File.readlines('./input').map { |line| line.strip }

rooms = raw_input.map do |raw|
  a, b = raw.split('[')
  toks = a.split('-')

  {
    checksum: b[0..-2],
    sector_id: toks.last.to_i,
    encrypted_name: toks[0..-2].join('-'),
  }
end

def checksum(encrypted_name)
  letters = encrypted_name.split('-').join

  counts = letters.each_char.reduce({}) do |memo, letter|
    memo[letter] ||= 0
    memo[letter] += 1
    memo
  end

  by_counts = counts.reduce([]) do |memo, (letter, count)|
    memo[count] ||= []
    memo[count] << letter
    memo[count].sort!
    memo
  end

  prioritized = by_counts.compact.reverse.flatten
  prioritized[0...5].join
end

total = 0
rooms.each do |r|
  total += r[:sector_id] if r[:checksum] == checksum(r[:encrypted_name])
end

puts total
