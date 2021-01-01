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
  a: 2,
  b: 0,
  c: 0,
  d: 0,
}

output = []
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
  when :out
    x = ins[:params][0]
    x = registers[x] if x.is_a?(Symbol)
    print x
    pc += 1
  else
    raise "Unrecognized command: #{ins.inspect}"
  end
end
