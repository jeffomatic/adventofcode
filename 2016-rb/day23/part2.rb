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
  a: 12,
  b: 0,
  c: 0,
  d: 0,
}

pc = 0
loop do
  break if pc < 0 || instructions.size <= pc

  # Optimization: perform lookahead for hardcoded multiplication and addition
  if pc == 4 &&
     instructions[4][:cmd] == :cpy &&
     instructions[7][:cmd] == :jnz &&
     instructions[9][:cmd] == :jnz
    sign = (instructions[5][:cmd] == :inc) ? 1 : -1
    registers[:a] += sign * (registers[:b] * registers[:d]).abs
    registers[:c] = 0
    registers[:d] = 0
    pc = 10
    next
  elsif pc == 13 &&
        instructions[15][:cmd] == :jnz
    sign = (instructions[14][:cmd] == :inc) ? 1 : -1
    registers[:c] += sign * registers[:d].abs
    registers[:d] = 0
    pc = 16
    next
  elsif pc == 20 &&
        instructions[20][:cmd] == :cpy &&
        instructions[23][:cmd] == :jnz &&
        instructions[25][:cmd] == :jnz
    sign = (instructions[21][:cmd] == :inc) ? 1 : -1
    registers[:a] += sign * (registers[:c] * 91).abs
    registers[:c] = 0
    registers[:d] = 0
    pc = 26
    next
  end

  ins = instructions[pc]

  case ins[:cmd]
  when :cpy
    x = ins[:params][0]
    x = registers[x] if x.is_a?(Symbol)
    y = ins[:params][1]
    registers[y] = x if registers.key?(y) # y may not be a register due to toggling
    pc += 1
  when :jnz
    x = ins[:params][0]
    x = registers[x] if x.is_a?(Symbol)
    y = ins[:params][1]
    y = registers[y] if y.is_a?(Symbol)

    if x != 0
      pc += y
    else
      pc += 1
    end
  when :inc
    registers[ins[:params].first] += 1
    pc += 1
  when :dec
    registers[ins[:params].first] -= 1
    pc += 1
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
    pc += 1
  else
    raise "Unrecognized command: #{ins.inspect}"
  end
end

puts registers.inspect
