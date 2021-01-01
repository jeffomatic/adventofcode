input = File.open('./input') { |f| f.read }.strip.split("\n")

def is_abba?(seq)
  seq[0] == seq[3] && seq[1] == seq[2] && seq[0] != seq[1]
end

def is_tls?(addr)
  has_abba = false
  hypernet = false
  seq = ''

  addr.each_char do |c|
    if c == '['
      hypernet = true
      seq = ''
      next
    elsif c == ']'
      hypernet = false
      seq = ''
      next
    end

    seq += c
    next if seq.size < 4
    seq = seq[-4..-1]

    next unless is_abba?(seq)

    return false if hypernet
    has_abba = true
  end

  has_abba
end

puts input.select { |addr| is_tls?(addr) }.count
