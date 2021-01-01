input = File.read('./input') { |f| f.read }.strip.split("\n")

sues = input.map do |line|
  toks = line.split(' ')

  attribs = {number: Integer(toks[1].chomp(':'))}

  toks = toks[2..-1]
  (toks.size / 2).times do |i|
    attrib = toks[2*i].chomp(':').to_sym
    number = Integer(toks[2*i+1].chomp(','))
    attribs[attrib] = number
  end

  attribs
end

target = {
  children: 3,
  cats: 7,
  samoyeds: 2,
  pomeranians: 3,
  akitas: 0,
  vizslas: 0,
  goldfish: 5,
  trees: 3,
  cars: 2,
  perfumes: 1,
}

sues.each do |attribs|
  catch :next_sue do
    attribs.each do |k, v|
      case k
      when :number
        next
      when :cats, :trees
        throw :next_sue unless v > target[k]
      when :pomeranians, :goldfish
        throw :next_sue unless v < target[k]
      else
        throw :next_sue unless v == target[k]
      end
    end

    puts attribs[:number]
    exit
  end
end
