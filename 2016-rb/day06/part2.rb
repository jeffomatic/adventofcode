input = File.readlines('./input').map(&:strip).reject { |line| line.empty? }
counts = input.first.size.times.reduce([]) { |memo| memo << {} }

input.each do |line|
  line.each_char.each_with_index do |letter, index|
    counts[index][letter] ||= 0
    counts[index][letter] += 1
  end
end

sorted_counts = counts.map do |slot_counts|
  slot_counts.each.sort do |a, b|
    a[1] <=> b[1]
  end
end

puts sorted_counts.map { |slot| slot.first.first }.join
