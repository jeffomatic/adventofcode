t = 0
loop do
  if t % 17 == 11 &&
     t % 19 == 9 &&
     t % 7 == 3 &&
     t % 13 == 2 &&
     t % 5 == 4 &&
     t % 3 == 0
    puts t
    exit
  end
  t += 1
end
