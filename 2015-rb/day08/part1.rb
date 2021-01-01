input = File.open('./input') { |f| f.read }.strip.split("\n")

def parsed_size(s)
  s = s[1...-1]
  size = 0
  i = 0
  backquote = false

  while i < s.size
    c = s[i]
    i += 1

    if backquote
      backquote = false

      if c == 'x'
        size += 1
        i += 2
        next
      end

      size += 1
      next
    end

    if c == '\\'
      backquote = true
      next
    end

    size += 1
  end

  size
end

raw = 0
parsed = 0

input.each do |s|
  raw += s.size
  parsed += parsed_size(s)
end

puts raw - parsed
