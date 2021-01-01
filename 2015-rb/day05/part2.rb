input = File.open('./input') { |f| f.read }.strip.split("\n")

def nice?(s)
  last = ''
  lastlast = ''
  has_repeat_with_gap = false
  has_repeated_pair = false
  first_occurence = {}

  s.each_char.each_with_index do |c, i|
    if lastlast == c
      return true if has_repeated_pair
      has_repeat_with_gap = true
    end

    if !has_repeated_pair
      k = [last, c]
      if !first_occurence.key?(k)
        first_occurence[k] = i
      elsif first_occurence[k] < (i - 1)
        return true if has_repeat_with_gap
        has_repeated_pair = true
      end
    end

    lastlast = last
    last = c
  end

  false
end

puts input.select { |s| nice?(s) }.count
