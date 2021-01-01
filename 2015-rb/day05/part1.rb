input = File.open('./input') { |f| f.read }.strip.split("\n")

VOWELS = ['a', 'e', 'i', 'o', 'u']
ILLEGAL = ['ab', 'cd', 'pq', 'xy']

def nice?(s)
  last = ''
  vowels = 0
  has_double = false

  s.each_char do |c|
    vowels += 1 if VOWELS.include?(c)

    if last == c
      has_double = true
    elsif ILLEGAL.include?(last + c)
      return false
    end

    last = c
  end

  return vowels >= 3 && has_double
end

puts input.select { |s| nice?(s) }.count
