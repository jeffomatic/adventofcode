require 'json'

input = File.read('./input')
input = JSON.parse(input)

def slurp_numbers(obj)
  nums = []
  case obj
  when Hash
    obj.values.each { |v| nums += slurp_numbers(v) }
  when Array
    obj.each { |v| nums += slurp_numbers(v) }
  when Numeric
    nums << obj
  end
  nums
end

puts slurp_numbers(input).reduce(0) { |memo, v| memo += v }
