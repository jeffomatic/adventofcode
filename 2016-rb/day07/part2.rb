input = File.open('./input') { |f| f.read }.strip.split("\n")

def is_aba?(seq)
  seq[0] == seq[2] && seq[0] != seq[1]
end

def is_ssl?(addr)
  babs = []
  hypernet_abas = []

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
    next if seq.size < 3
    seq = seq[-3..-1]

    if is_aba?(seq)
      if hypernet
        hypernet_abas << seq
      else
        babs << seq[1] + seq[0] + seq[1]
      end
    end
  end

  !(babs & hypernet_abas).empty?
end

puts input.select { |addr| is_ssl?(addr) }.count
