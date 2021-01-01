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

def decrypt_word(word, rot)
  rot %= 26
  word.each_char.map { |letter|
    c = letter.ord + rot
    c -= 26 if c > 'z'.ord
    c.chr
  }.join
end

rooms.each do |r|
  name = r[:encrypted_name].split('-').map { |w|
    decrypt_word(w, r[:sector_id])
  }.join(' ')
  puts "#{name}: #{r[:sector_id]}"
end
