input = File.open('./input') { |f| f.read }.strip.split("\n")

input << "3176 -> b"

def parse_ident(tok)
  {
    type: :literal,
    args: [Integer(tok)],
  }
rescue ArgumentError
  {
    type: :read,
    args: [tok],
  }
end

nodes = {}

input.each do |c|
  expr, dest = c.split(' -> ')
  toks = expr.split(' ')

  case toks.count
  when 1
    nodes[dest] = parse_ident(toks.first)
  when 2
    nodes[dest] = {type: :negate, args: [parse_ident(toks.last)]}
  when 3
    nodes[dest] = {
      type: toks[1].downcase.to_sym,
      args: [
        parse_ident(toks[0]),
        parse_ident(toks[2]),
      ],
    }
  end
end

def eval(op, nodes, cache = {})
  args = op[:args]
  case op[:type]
  when :literal
    args.first
  when :read
    cache[args.first] ||= eval(nodes[args.first], nodes, cache)
  when :negate
    ~eval(args.first, nodes, cache) & 65535
  when :or
    eval(args.first, nodes, cache) | eval(args.last, nodes, cache)
  when :and
    eval(args.first, nodes, cache) & eval(args.last, nodes, cache)
  when :lshift
    (eval(args.first, nodes, cache) & 65535) << eval(args.last, nodes, cache)
  when :rshift
    (eval(args.first, nodes, cache) & 65535) >> eval(args.last, nodes, cache)
  else
    raise "Invalid operator: #{op.inspect}"
  end
end

puts eval(nodes['a'], nodes)
