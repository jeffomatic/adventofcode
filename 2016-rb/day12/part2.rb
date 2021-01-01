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
  a: 0,
  b: 0,
  c: 1,
  d: 0,
}

pc = 0

loop do
  ins = instructions[pc]
  raise "Invalid PC: #{pc}" if ins.nil?

  case ins[:cmd]
  when :cpy
    v = ins[:params][0]
    v = registers[v] if v.is_a?(Symbol)
    dst = ins[:params][1]
    registers[dst] = v
  when :jnz
    v = ins[:params][0]
    v = registers[v] if v.is_a?(Symbol)
    if v != 0
      pc += ins[:params][1]
      next
    end
  when :inc
    registers[ins[:params].first] += 1
  when :dec
    registers[ins[:params].first] -= 1
  else
    "Unrecognized command: #{ins.inspect}"
  end

  pc += 1
  break if pc == instructions.size
end

puts registers.inspect
