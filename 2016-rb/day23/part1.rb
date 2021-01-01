input = File.open('./input') { |f| f.read }.strip.split("\n")

instructions = input.map do |line|
  toks = line.split
  cmd = toks[0].to_sym
  params = toks[1..-1].map do |tok|
    if tok.match /^-?(\d+)/
      tok.to_i
    else
      tok.to_sym
    end
  end

  {
    cmd: cmd,
    params: params,
  }
end

registers = {
  a: 7,
  b: 0,
  c: 0,
  d: 0,
}

pc = 0
loop do
  break if pc < 0 || instructions.size <= pc

  ins = instructions[pc]

  case ins[:cmd]
  when :cpy
    x = ins[:params][0]
    x = registers[x] if x.is_a?(Symbol)
    y = ins[:params][1]
    registers[y] = x if registers.key?(y) # y may not be a register due to toggling
  when :jnz
    x = ins[:params][0]
    x = registers[x] if x.is_a?(Symbol)
    y = ins[:params][1]
    y = registers[y] if y.is_a?(Symbol)

    if x != 0
      pc += y
      next
    end
  when :inc
    registers[ins[:params].first] += 1
  when :dec
    registers[ins[:params].first] -= 1
  when :tgl
    x = ins[:params][0]
    x = registers[x] if x.is_a?(Symbol)
    i = pc + x
    if 0 <= i && i < instructions.size
      target = instructions[i]
      case target[:cmd]
      when :cpy then target[:cmd] = :jnz
      when :jnz then target[:cmd] = :cpy
      when :inc then target[:cmd] = :dec
      when :dec then target[:cmd] = :inc
      when :tgl then target[:cmd] = :inc
      else raise "Unrecognized command: #{target}"
      end
    end
  else
    raise "Unrecognized command: #{ins.inspect}"
  end

  pc += 1
end

puts registers.inspect
