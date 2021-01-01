require 'digest'

input = 'abbhdwsy'

def hash(prefix, index)
  Digest::MD5.hexdigest("#{prefix}#{index}")
end

password = ''
i = 0
while password.size < 8
  h = hash(input, i)
  password += h[5] if h[0..4] == '00000'
  i += 1
end

puts password
