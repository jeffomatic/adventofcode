require 'digest'

input = 'abbhdwsy'

def hash(prefix, index)
  Digest::MD5.hexdigest("#{prefix}#{index}")
end

password = []
i = 0
while password.compact.size < 8
  h = hash(input, i)
  index = Integer(h[5]) rescue nil

  if index && index < 8 && password[index].nil? && h[0..4] == '00000'
    password[index] = h[6]
  end

  i += 1
end

puts password.join
