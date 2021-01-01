# new disc: t + 7 = 0 mod 11, or t = 4 mod 11

t = 0
loop do
  if t % 17 == 11 &&
     t % 19 == 9 &&
     t % 7 == 3 &&
     t % 13 == 2 &&
     t % 5 == 4 &&
     t % 3 == 0 &&
     t % 11 == 4
    puts t
    exit
  end
  t += 1
end
