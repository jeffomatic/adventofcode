require 'digest'

input = 'iwrupvqb'
i = 0
loop do
  s = Digest::MD5.hexdigest(input + i.to_s)
  if s[0..5] == '000000'
    puts i
    exit
  end
  i += 1
end
